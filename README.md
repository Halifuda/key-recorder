# keyboard recorder

> About Language: This is a Chinese README. For English, see README-en.md.

## 简介
key-recorder是一个非常简单的键盘输入记录器。打开程序后，其会安静地记录用户在键盘上的输入行为。此程序可以记录81个常见键位。

当接受到一个SIGINT信号（或者CTRLC信号），程序会将从启动到接受到信号之间记录的键盘事件存入`./records`目录下新建的一个txt文件。

txt文件命名格式为："record-{starttime}-{endtime}"。

txt文件的第一行是starttime，第二行是endtime，第三行是81个整数，表示每个键在这段时间里被按下的次数，第四行是在这段时间里按下的键的序列。

其中第三行每一个整数表示哪一个键位，您可以在`main.rs`的第347行处找到。

## 依赖
此项目主要依赖于`device_query-1.0.0`, `chrono-0.4.19`, `ctrlc-3.2.1`。由于`divice_query`是一个跨平台的库，因此本项目也是跨平台的。

## 局限
这不是一个全局键盘监听器。

某些其它程序（尤其是全屏程序）正在运行时，key-recorder是无法工作的。我在win10平台上测试了包括“原神”、“永劫无间”在内的多种游戏和程序，均无法工作。

我目前怀疑这是由于在windows下，这些程序对键盘独占，或者这些程序在运行的时候强行挂起了key-recorder。

这个问题有可能可以用Winapi中的Hook来解决。我打算依然用rust和winapi-rs完成，但用C++或.NET应该会更简单。

我还没有研究如何在Linux下解决这类问题，但应该比在windows下简单。

## 扩展
若您感兴趣，您可以对记下的txt文件做您想做的处理。

您也可以写出其他程序，将这个程序运行在后台进程。

由于我的本意是一个全局键盘监听器，因此我目前没有在这个项目上进行这些工作。
