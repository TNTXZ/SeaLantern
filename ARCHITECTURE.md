Sea Lantern — 项目架构
======================

这是什么
--------

Sea Lantern 是一个 Minecraft 服务器管理工具。
Tauri 2 做壳，Rust 管后端，Vue 3 画前端。

它现在能用，但远远没有完成。架构是开放的，代码是干净的，剩下的交给你。


技术选型
--------

    前端    Vue 3 + TypeScript + Vite + Pinia
    后端    Rust + Tauri 2
    样式    纯 CSS
    通信    Tauri invoke（前端调 Rust 函数，直接拿返回值）

没有 Electron，没有 Node 后端，没有 Webpack。启动快，体积小，内存省。


目录结构
--------

    sea-lantern/
    │
    ├── src/                                  前端代码
    │   │
    │   ├── api/                              和 Rust 后端通信的封装层
    │   │   ├── tauri.ts                      基础 invoke 封装，所有 API 都经过这里
    │   │   ├── server.ts                     服务器的增删改查、启动停止、日志拉取
    │   │   ├── java.ts                       Java 环境检测
    │   │   ├── config.ts                     server.properties 读写
    │   │   ├── player.ts                     白名单、封禁、OP 操作
    │   │   ├── settings.ts                   应用设置读写
    │   │   └── system.ts                     系统信息、文件选择对话框
    │   │
    │   ├── components/
    │   │   ├── common/                       通用 UI 组件，整个项目的积木块
    │   │   │   ├── SLButton.vue              按钮，支持 primary / secondary / ghost / danger
    │   │   │   ├── SLCard.vue                卡片容器，带毛玻璃效果
    │   │   │   ├── SLInput.vue               输入框，支持前缀后缀插槽
    │   │   │   ├── SLSelect.vue              下拉选择
    │   │   │   ├── SLSwitch.vue              开关
    │   │   │   ├── SLModal.vue               弹窗
    │   │   │   ├── SLProgress.vue            进度条
    │   │   │   └── SLBadge.vue               状态标签
    │   │   │
    │   │   └── layout/                       页面骨架
    │   │       ├── AppLayout.vue             总布局：左侧栏 + 右侧内容区
    │   │       ├── AppSidebar.vue            侧边导航栏
    │   │       └── AppHeader.vue             顶部标题栏
    │   │
    │   ├── views/                            页面，每个路由对应一个
    │   │   ├── HomeView.vue                  首页：服务器列表、系统状态、最近告警
    │   │   ├── CreateServerView.vue          创建/导入服务器：Java 检测、JAR 选择
    │   │   ├── ConsoleView.vue               控制台：实时日志、命令输入、快捷指令
    │   │   ├── ConfigView.vue                配置编辑：server.properties 可视化编辑
    │   │   ├── PlayerView.vue                玩家管理：在线列表、白名单、封禁、OP
    │   │   ├── SettingsView.vue              应用设置
    │   │   └── AboutView.vue                 关于页面 + 贡献者墙
    │   │
    │   ├── stores/                           Pinia 状态管理
    │   │   ├── index.ts                      Pinia 实例
    │   │   ├── serverStore.ts                服务器列表和运行状态
    │   │   ├── consoleStore.ts               控制台日志，切换页面不丢失
    │   │   └── uiStore.ts                    界面状态（侧栏折叠等）
    │   │
    │   ├── styles/                           全局样式
    │   │   ├── variables.css                 颜色、间距、圆角、字体、阴影
    │   │   ├── reset.css                     浏览器样式重置
    │   │   ├── typography.css                排版
    │   │   ├── animations.css                动画关键帧
    │   │   └── glass.css                     毛玻璃效果
    │   │
    │   ├── router/index.ts                   路由表
    │   ├── App.vue                           根组件
    │   ├── main.ts                           入口
    │   └── style.css                         样式汇总导入
    │
    ├── src-tauri/                            后端代码
    │   ├── src/
    │   │   ├── commands/                     Tauri 命令，前端 invoke 调的就是这些
    │   │   │   ├── mod.rs                    模块导出
    │   │   │   ├── server.rs                 创建、启动、停止、删除、取日志
    │   │   │   ├── java.rs                   Java 检测和校验
    │   │   │   ├── config.rs                 配置文件读写
    │   │   │   ├── player.rs                 玩家管理（向运行中的服务器发 MC 命令）
    │   │   │   ├── settings.rs               应用设置
    │   │   │   └── system.rs                 系统信息、原生文件选择对话框
    │   │   │
    │   │   ├── services/                     业务逻辑层
    │   │   │   ├── mod.rs
    │   │   │   ├── server_manager.rs         进程生命周期、stdout/stderr 线程、JSON 持久化
    │   │   │   ├── java_detector.rs          全盘 Java 扫描器
    │   │   │   ├── config_parser.rs          .properties 解析器，带中文描述和分类
    │   │   │   ├── player_manager.rs         whitelist / banned-players / ops 文件读取
    │   │   │   └── settings_manager.rs       应用设置 JSON 持久化
    │   │   │
    │   │   ├── models/                       数据结构
    │   │   │   ├── mod.rs
    │   │   │   ├── server.rs                 ServerInstance, ServerStatus
    │   │   │   ├── config.rs                 ConfigEntry, ServerProperties
    │   │   │   └── settings.rs               AppSettings
    │   │   │
    │   │   ├── utils/mod.rs                  工具模块，目前为空
    │   │   ├── lib.rs                        Tauri 入口：插件注册、命令注册、关闭事件
    │   │   └── main.rs                       主函数
    │   │
    │   ├── capabilities/default.json         Tauri 权限配置
    │   ├── Cargo.toml                        Rust 依赖
    │   └── tauri.conf.json                   窗口大小、标题
    │
    ├── ARCHITECTURE.md                       你正在看的这个文件
    └── README.md                             项目介绍


数据流
------

用户点了个按钮。

    Vue 组件
      -> src/api/*.ts（类型安全的封装）
        -> tauri::invoke()
          -> src-tauri/src/commands/*.rs（参数校验，分发）
            -> src-tauri/src/services/*.rs（实际逻辑）
              -> 文件系统 / 子进程 / 网络
            <- Result<T, String>
          <- JSON
        <- Promise<T>
      <- 响应式更新

就是这样。前端不碰文件系统，后端不碰 DOM。各干各的。


已经做了什么
------------

服务器管理
    导入 JAR 文件创建服务器，一键启动和停止。
    数据保存到 JSON，重启软件不丢失。

实时控制台
    后端用独立线程读 stdout 和 stderr。
    前端每 800ms 轮询拉新日志。
    支持命令输入、Tab 补全、上下键历史、快捷指令按钮。
    日志存在全局 store 里，切页面不丢。

Java 检测
    启动时扫描 A 到 Z 所有盘符。
    递归搜索常见安装路径，包括 .minecraft/runtime 里 MC 自带的 Java。
    按版本号排序，标记推荐。

配置编辑
    读取 server.properties，解析成带描述和分类的结构化数据。
    布尔值用开关，枚举用下拉，数字和字符串用输入框。
    改完直接写回文件。

玩家管理
    读取 whitelist.json / banned-players.json / ops.json 显示列表。
    添加和移除通过向运行中的服务器发送 MC 命令实现。
    解析日志判断在线玩家。

设置
    关闭软件时自动停止所有服务器（默认开启）。
    自动同意 EULA。
    默认内存、端口、JVM 参数，全部可配。

文件选择
    通过 tauri-plugin-dialog 调用系统原生文件对话框。


还没做的
--------

这些功能的位置都预留好了，骨架是现成的，等你来写。当然，不局限于这些，你想到什么，就做什么

    下载中心        下载服务端核心，插件，模组
    备份管理        世界存档的增量备份和还原
    内网穿透        集成 FRP
    定时任务        自动重启、定时备份、定时执行命令
    资源管理        从 Modrinth / CurseForge 搜索安装插件和 Mod
    暗色主题        CSS 变量都准备好了，加一套 dark 的值就行
    国际化          目前全是中文硬编码，可以抽成语言文件
    等等......


怎么加一个新功能
----------------

假设你要加一个「备份管理」功能。

后端：
    1. src-tauri/src/services/ 下建 backup_manager.rs，写逻辑
        2. src-tauri/src/commands/ 下建 backup.rs，写 Tauri 命令
        3. 在 commands/mod.rs 里加 pub mod backup
        4. 在 lib.rs 的 generate_handler! 宏里注册命令

前端：
    1. src/api/ 下建 backup.ts，封装 invoke 调用
    2. src/views/ 下建 BackupView.vue，画页面
    3. src/router/index.ts 里加路由
    4. AppSidebar.vue 的 navItems 数组里加一项

前后端各三个文件，路由和侧栏各改一行。


怎么改样式
----------

所有颜色、间距、圆角、阴影、字体都在 src/styles/variables.css 里。
改那个文件就能改全局风格。

毛玻璃效果在 src/styles/glass.css 里。
不喜欢毛玻璃，把 backdrop-filter 删了换成纯色就行。

通用组件在 src/components/common/ 里。
每个组件都是独立的，改一个不影响其他的。


怎么跑起来
----------

装好 Node.js 和 Rust。

    npm install
    npm run tauri dev

第一次会编译 Rust，要几分钟。之后热更新很快。

构建发布版：

    npm run tauri build

产物在 src-tauri/target/release/ 里。