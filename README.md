海晶灯（Sea Lantern）
===========

Minecraft 服务器管理工具。

Tauri 2 + Rust + Vue 3。


能干什么
--------

导入一个服务端 JAR 文件，选一个 Java，点启动。就这么简单。

控制台实时看日志，直接输命令。
server.properties 图形化编辑，不用手改文件。
白名单、封禁、OP 一键管理。
关软件的时候自动帮你停服务器，不会丢存档。


跑起来
------

需要 Node.js 20+ 和 Rust 1.70+。

    git clone https://gitee.com/你的用户名/sea-lantern.git
    cd sea-lantern
    npm install
    npm run tauri dev


项目结构
--------

前端在 src/，后端在 src-tauri/src/。

详细的目录说明和架构设计见 ARCHITECTURE.md。


参与开发
--------

下载中心、备份管理、内网穿透、定时任务、暗色主题、多语言，
这些功能的位置都预留好了，代码骨架是现成的，等你来写。

界面也是。颜色在 CSS 变量里，组件是独立的，不喜欢就换。
想做个主题皮肤？做。想把整个布局推翻重来？也行。

怎么贡献：

    1. Fork
    2. 建分支写代码
    3. 提 Pull Request
    4. 你的名字会出现在关于页面的贡献者墙上

不会写代码也行。
说你想要什么功能，或者画个 UI 草图发出来，都算贡献。


License
-------

GPLv3