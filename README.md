# 程序题目：基于领域特定语言的客服机器人设计与实现

这是我们程序设计实验大作业

我们将分步进行，下属每个章节将完成一个任务，总共四个章节

1. 定义领域特定脚本语言（DSL）的语法：根据客服机器人自动应答逻辑的要求，设计语言的关键词、语法规则、表达式组织和控制结构等。
2. 实现解释器：根据你定义的语法规则，编写一个解释器程序，能够解析并执行DSL脚本。解释器应能处理用户输入的脚本，按照脚本描述的逻辑给出相应的应答。
3. 编写几个脚本范例：根据不同的应答逻辑，编写几个脚本范例，包括不同的场景和交互情境。这些脚本范例应包含DSL语言的语法和语义的示例，以便测试和演示解释器的行为表现。
4. 测试和演示：用编写的脚本范例测试解释器的功能和正确性。检查解释器是否能正确解析和执行DSL语言的脚本，并根据逻辑要求给出正确的应答。可以使用命令行界面或其他形式的输入输出进行测试和演示。

## 0. 用户使用手册

```
$ cargo build	 #项目构建
$ cargo run		 #项目运行
$ cargo clean	 #清除缓存与编译文件
$ cargo test	 #项目测试
```

## 1. DSL语法定义

客服机器人DSL将包含以下关键字：`STAET`, `END`, `MATCH`, `RESPONSE`和`UNKNOWN`. 此外，我们还会添加支持简单条件判断的`CASE`, `DEFAULT`关键字，并支持和`MATCH`的组合完成`MATCH`的嵌套。。这将使得我们的脚本具有更高的灵活度。

1. `START` 和 `END`: 定义一个块的开始和结束。所有的脚本必须在`START`和`END`关键字之间。
2. `MATCH`: 匹配用户输入的特定模式。
3. `RESPONSE`: 紧跟在`MATCH`或`CASE`关键字后，指定机器人对匹配到的输入的响应。
4. `UNKNOWN`: 在没有匹配到任何输入时提供的默认回复。
5. `CASE`和`DEFAULT`: 组成一种判断结构。在`MATCH`之后，每个`CASE`后面跟随判断条件和对应的应答。`DEFAULT`后面跟随默认的应答。每个`CASE`之后，还可以通过`START`和`MATCH`的组合完成`MATCH`的嵌套。

下面是一个脚本的示例：

```
STAET
  MATCH "我需要帮助"
  RESPONSE "当然，我很乐意帮助你。你遇到什么问题了呢？"

  MATCH "我的订单状态是什么"
  RESPONSE "让我为你查询。请你提供下订单号。"

    CASE "我的订单号是12345"
    RESPONSE "已查询到该订单号，请问需要什么服务"
      STAET
        MATCH "我想改变配送地址"
        RESPONSE "好的，你想更改为哪个地址？"
          CASE "我想改为100号大街"
          RESPONSE "你的配送地址已经更改为100号大街。"
          DEFAULT
          RESPONSE "对不起，这个地址我们无法配送。"
      END

    DEFAULT
    RESPONSE "对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。"

  MATCH "我无法登录我的账户"
  RESPONSE "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?"

  UNKNOWN
  RESPONSE "抱歉，我不明白你的问题。能否请你再详细描述一下？"
END
```

- 当用户输入"我需要帮助"，机器人会回复"当然，我很乐意帮助你。你遇到什么问题了呢？"
- 当用户输入"我的订单状态是什么"，机器人会回复"让我为你查询。请你提供下订单号。" 并且在用户输入订单号后，如果订单号为"12345"，机器人会执行与之对应的CASE条件语句。
- 在所有`CASE`条件都不满足时(即订单号不是"12345")，机器人会执行`DEFAULT`后的语句，即回复"对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。"
- 最后，当用户输入不在所有`MATCH`规则的匹配范围时，机器人会执行`UNKNOWN`后的语句，表达它无法理解用户的问题。
- 同时脚本编写者可编写正则表达式进行模糊匹配

## 2.实现解释器

我想用rust实现，rust的模式匹配特性或许更方便实现这种格式

程序框架设计如下

````
.
├── Cargo.lock 								//记录依赖包元数据，cargo自动维护
├── Cargo.toml								//cargo项目管理脚本，维护项目信息和依赖包
├── README.md									//项目简介，也是实验过程记录
├── scripts					//脚本样例		
│   ├── bad_script.txt
│   ├── script.txt
│   ├── script_1.txt
│   └── script_regex.txt
└── src							//项目源码
    ├── block.rs							//block结构的定义与转化
    ├── command.rs						//command枚举变量的定义与转化
    ├── main.rs								//程序入口，处理用户输入和读取文件，程序逻辑实现
    └── script.rs							//脚本运行
````

### 模块一：将脚本语言转化为command向量

command枚举变量结构的定义

```rust
pub enum Command {
    START,
    MATCH(String),
    RESPONSE(String),
    UNKNOWN,
    CASE(String),
    DEFAULT,
    END,
}
```

````rust
// 将一行字符串转化为Command枚举类型 
fn parse_line_to_cmd(line: &str) -> Option<Command> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // 使用正则表达式对传入脚本行进行匹配
    let re = Regex::new(r#"(\w+)\s*(".*")?"#).unwrap();
    let cap = re.captures(line)?;

    // 模式匹配，第一块为脚本命令，第二块为脚本信息
    let cmd = match cap.get(1)?.as_str() {
        "START" => Command::START,
        "MATCH" => Command::MATCH(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "RESPONSE" => Command::RESPONSE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "CASE" => Command::CASE(cap.get(2)?.as_str().trim_matches('"').to_string()),
        "DEFAULT" => Command::DEFAULT,
        "UNKNOWN" => Command::UNKNOWN,
        "END" => Command::END,
        _ => {
            panic!("bad command {}", line)
        },
    };

    Some(cmd)
}
pub fn print_command(ref cmd : Command)// 打印Command
// 打开脚本将脚本中的自然语言转化为Command向量
pub fn parse_file_to_cmds(filename: &str) -> io::Result<Vec<Command>> 
````

### 模块二：将command向量转化为block

block结构体的定义

![image-20231127110337197](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311271103265.png)

````rust
//主块定义，包含一个MatchBlock向量和一个UnknowingBlock
pub struct Block {
    pub matches: Vec<MatchBlock>,
    pub unknowing: UnknowingBlock,
}
//Match块，
pub struct MatchBlock {
    pub mtch: String,
    pub response: String,
    pub cases: Option<Vec<CaseBlock>>,
    pub default: Option<String>,
}
pub struct CaseBlock {
    pub case: String,
    pub response: String,
    pub matches: Option<Vec<MatchBlock>>,
}
pub struct UnknowingBlock {
    pub response: String,
}
````

````rust

enum Status{
    INITIAL,
    START,
    MATCH,
    MATCHANSWER,
    CASE,
    CASEANSWER,
    DEFAULT,
    DEFAULTANSWER,
    UNKNOWN,
    UNKNOWNANSWER,
    END
}
````

````rust
//将command向量转化为block
pub fn parse_commands_to_blocks(commands: Vec<Command>) -> Result<Block, &'static str>
````

经过parse_file_to_cmds将脚本中的脚本语言转化为Command向量，再通过parse_commands_to_blocks将Command向量转化成Block结构体

下图是parse_commands_to_blocks流程图的转移，初始状态为INITIAL

图中方框中为状态，直线或曲线上的文字为转移状态的指令，若该状态接收到无法解释的命令则会返回error

![image-20231122220633656](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311222206689.png)

实例脚本转化完成后如下

````rust
Block {
    matches: [
        MatchBlock {
            mtch: "我需要帮助",
            response: "当然，我很乐意帮助你。你遇到什么问题了呢？",
            cases: None,
            default: None,
        },
        MatchBlock {
            mtch: "我的订单状态是什么",
            response: "让我为你查询。请你提供下订单号。",
            cases: Some([
                CaseBlock {
                    case: "我的订单号是12345",
                    response: "已查询到该地址，请问需要什么服务？",
                    matches: Some([
                        MatchBlock {
                            mtch: "我想改变配送地址",
                            response: "好的，你想更改为哪个地址？",
                            cases: Some([
                                CaseBlock {
                                    case: "我想改为100号大街",
                                    response: "你的配送地址已经更改为100号大街。",
                                    matches: None,
                                },
                            ]),
                            default: Some("对不起，这个地址我们无法配送。"),
                        },
                    ]),
                },
            ]),
            default: Some("对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。"),
        },
        MatchBlock {
            mtch: "我无法登录我的账户",
            response: "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?",
            cases: None,
            default: None,
        },
    ],
    unknowing: UnknowingBlock {
        response: "抱歉，我不明白你的问题。能否请你再详细描述一下？",
    },
}
````

### 模块三：通过block运行脚本

current_match记录上次匹配到的match块，current_case记录上次匹配到的case块，str_match通过正则表达式规则进行正则匹配

通过current_case和current_match两个变量是否为空，可隐式的将状态根据上次match结果分为

1. match状态
2. case状态
3. 空状态

match状态表明上次匹配匹配到了match块，此状态只能匹配current_match下的case，若匹配失败则默认匹配current_match下的default

case状态表明上次匹配到case块，此状态只能匹配current_case状态下的matches，若匹配失败则进入unknow块

空状态表明上次匹配失败，或者刚进入程序，只能匹配主块的matches

````rust
// 判断给定字符串是否符合正则表达式
fn str_match(str1: &str, reg: &str) -> bool{
    let re = RegexBuilder::new(reg.trim()).unicode(true)
    .build()
    .unwrap();
    println!("{} {} {}", str1.trim(), reg.trim(), re.is_match(str1.trim()));
    re.is_match(str1.trim())
}
// 传入脚本对应的Block，运行脚本
pub fn execute(m_block :Block){
    let Block {
        matches,
        unknowing,
    } = m_block;
    let mut current_match: Option<MatchBlock> = None;
    let mut current_case: Option<CaseBlock> = None;
    'outer: loop{
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("无法读取行");
        if let Some(case_block) = current_case{
            match case_block.matches{
                Some(match_vec) =>{
                    for match_block in match_vec.iter(){
                        if str_match(line.as_str(), match_block.mtch.as_str()){
                            println!("{}", match_block.response);
                            current_match = Some(match_block.to_owned());
                            current_case = None;
                            continue 'outer;
                        }
                    }
                    println!("{}", unknowing.response);
                    current_match = None;
                    current_case = None;
                    continue 'outer;
                },
                None =>{
                    current_case = None;
                }
            }
        }
        if let Some(match_block) = current_match{
            match match_block.cases{
                Some(case_vec) => {
                    for case_block in case_vec.iter(){
                        if str_match(line.as_str(), case_block.case.as_str()){
                            println!("{}", case_block.response);
                            current_match = None;
                            current_case = Some(case_block.to_owned());
                            continue 'outer;
                        }
                    }
                    println!("{}", match_block.default.unwrap());
                    current_match = None;
                    current_case = None;
                    continue 'outer;
                }
                None => {
                    current_match = None;
                }
            }
        }
        for match_block in matches.iter(){
            if str_match(line.as_str(), match_block.mtch.as_str()){
                println!("{}", match_block.response);
                current_match = Some(match_block.to_owned());
                current_case = None;
                continue 'outer;
            }
        }
        println!("{}", unknowing.response);
        current_match = None;
        current_case = None;
        continue 'outer;
    }
}
````

## 3.脚本范例编写

我编写了三个测试脚本位于scripts文件夹

Script.txt

实例脚本，初始测试脚本

````
START
  MATCH "我需要帮助"
  RESPONSE "当然，我很乐意帮助你。你遇到什么问题了呢？"

  MATCH "我的订单状态是什么"
  RESPONSE "让我为你查询。请你提供下订单号。"

  CASE "我的订单号是12345"
  RESPONSE "已查询到该地址，请问需要什么服务？"
    START
      MATCH "我想改变配送地址"
      RESPONSE "好的，你想更改为哪个地址？"
      CASE "我想改为100号大街"
      RESPONSE "你的配送地址已经更改为100号大街。"
      DEFAULT
      RESPONSE "对不起，这个地址我们无法配送。"
    END

  DEFAULT
  RESPONSE "对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。"

  MATCH "我无法登录我的账户"
  RESPONSE "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?"

  UNKNOWN
  RESPONSE "抱歉，我不明白你的问题。能否请你再详细描述一下？"
END
````

script_1.txt

多重嵌套测试脚本，用于测试多重子块嵌套情形

````
START
  MATCH "我这里有些问题"
  RESPONSE "您有什么问题呢"

  MATCH "我买的衣服大了一号"
  RESPONSE "请问您有没有撕牌？"

  CASE "还没撕"
  RESPONSE "那衣服有没有破损呢？"
    START
      MATCH "没有破损"
      RESPONSE "那可以退换货，请问您想要大一码的还是想直接退货呢？"

        CASE "直接退货"
        RESPONSE "好的，等我们收到衣服，钱将原路返回"

        CASE "换一个大的"
        RESPONSE "好的，请问款式和收货地址还是原来的吗？"
            START
                MATCH "对的，不变"
                RESPONSE "好的，您已提交换货申请"
            END

        DEFAULT
        RESPONSE "对不起，这个地址我们无法配送。"

      MATCH "让我弄破了一点"
      RESPONSE "弄破了就不能退换货了"
    END

  CASE "已经撕了"
  RESPONSE "那您不可以退了哦"

DEFAULT
RESPONSE "听不懂您在讲什么"

  MATCH "你们的衣服质量有问题"
  RESPONSE "没啥问题"

  UNKNOWN
  RESPONSE "抱歉，我不明白你的问题。能否请你再详细描述一下？"
END
````

Script_regex.txt

正则表达式测试脚本，用于测试正则表达式是否生效

````
START
  MATCH ".*物流.*"
  RESPONSE "暂时查询不到您的物流"

  MATCH ".*尺码.*"
  RESPONSE "请您说您的尺码，我给您推荐型号"

  CASE ".*180m140kg.*"
  RESPONSE "XL🐎"

  DEFAULT
  RESPONSE "对不起我也不知道你该穿多大的"

  MATCH ".*无法登录.*"
  RESPONSE "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?"

  UNKNOWN
  RESPONSE "抱歉，我不明白你的问题。能否请你再详细描述一下？"
END
````

## 4.测试桩与单元测试

通过命令cargo test运行所有测试

![image-20231123000551385](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311230005431.png)

rust有很好的测试系统，可以将测试模块集成到源码中，总共存在七个测试，各自位于所属的单元下

例如test_parse_commands_to_blocks和test_bad_parse_commands_to_blocks

````rust
#[test]
fn test_parse_commands_to_blocks() {
    let cmd_vec = match parse_file_to_cmds("scripts/script.txt") {
        Ok(vec) => vec,
        Err(error) => panic!("error: {}", error),
    };
    let m_block = match parse_commands_to_blocks(cmd_vec) {
        Ok(block) => block,
        Err(error) => panic!("error: {}", error)
    };
    let m_str = format!("{:?}", m_block.clone());
    let answer_str = r#"Block { matches: [MatchBlock { mtch: "我需要帮助", response: "当然，我很乐意帮助你。你遇到什么问题了呢？", cases: None, default: None }, MatchBlock { mtch: "我的订单状态是什么", response: "让我为你查询。请你提供下订单号。", cases: Some([CaseBlock { case: "我的订单号是12345", response: "已查询到该地址，请问需要什么服务？", matches: Some([MatchBlock { mtch: "我想改变配送地址", response: "好的，你想更改为哪个地址？", cases: Some([CaseBlock { case: "我想改为100号大街", response: "你的配送地址已经更改为100号大街。", matches: None }]), default: Some("对不起，这个地址我们无法配送。") }]) }]), default: Some("对不起，我无法查询到你提供的订单信息，请检查订单号是否正确。") }, MatchBlock { mtch: "我无法登录我的账户", response: "抱歉给您带来不便。你是否忘记了密码，或被告知你的账户被冻结了?", cases: None, default: None }], unknowing: UnknowingBlock { response: "抱歉，我不明白你的问题。能否请你再详细描述一下？" } }"#;
    assert_eq!(m_str, answer_str);
}
````

````rust
#[test]
#[should_panic (expected = "error: 接受Command::MATCH处于错误的状态")]
fn test_bad_parse_commands_to_blocks() {
    let cmd_vec = match parse_file_to_cmds("scripts/bad_script.txt") {
        Ok(vec) => vec,
        Err(error) => panic!("error: {}", error),
    };
    let m_block = match parse_commands_to_blocks(cmd_vec) {
        Ok(block) => block,
        Err(error) => panic!("error: {}", error)
    };
}

````

分别测试parse_commands_to_blocks可以将commands正确转化为blocks，接收到不合法的commands后会抛出错误

### 5. 脚本测试

通过运行cargo run对项目进行编译运行

然后输入脚本路径，路径可以选择项目根目录的相对路径或者绝对路径

多重嵌套程序测试成功

![image-20231123001203121](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311230012161.png)

正则表达式模糊匹配成功

![image-20231123001419576](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311230014614.png)

![image-20231123001541843](https://raw.githubusercontent.com/tangdouer1005/tangdourercdn/main/mac/202311230015873.png)
