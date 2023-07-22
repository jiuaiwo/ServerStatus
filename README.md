<p align="center">
  <a href="https://github.com/zdz/ServerStatus-Rust">
    <h1 align="center">✨ Rust 版 ServerStatus 云探针</h1>
  </a>
</p>

<div align="center">
    <p>
        <a href="https://github.com/zdz/ServerStatus-Rust/actions/workflows/docker.yml">
            <img src="https://github.com/zdz/ServerStatus-Rust/actions/workflows/docker.yml/badge.svg"
                  alt="Docker">
        </a>
        <a href="https://github.com/zdz/ServerStatus-Rust/actions/workflows/release.yml">
            <img src="https://github.com/zdz/ServerStatus-Rust/actions/workflows/release.yml/badge.svg" alt="Release"></a>
        <a href="https://github.com/zdz/ServerStatus-Rust/issues">
            <img src="https://img.shields.io/github/issues/zdz/ServerStatus-Rust"
                  alt="GitHub issues">
        </a>
        <a href="https://github.com/zdz/ServerStatus-Rust/discussions">
            <img src="https://img.shields.io/github/discussions/zdz/ServerStatus-Rust"
                  alt="GitHub Discussions">
        </a>
        <a href="https://github.com/zdz/ServerStatus-Rust/releases">
            <img src="https://img.shields.io/github/v/release/zdz/ServerStatus-Rust"
                  alt="GitHub release (latest SemVer)">
        </a>
        <a href="https://github.com/zdz/ServerStatus-Rust/releases">
            <img src="https://img.shields.io/github/downloads/zdz/ServerStatus-Rust/total" alt="GitHub all releases">
        </a>
    </p>
</div>

<img width="1317" alt="image" src="https://user-images.githubusercontent.com/152173/206825541-6eaeb856-0c03-479a-b07e-006b60b41c02.png">


原始的 https://github.com/zdz/ServerStatus-Rust 

自己复制上来就是图自己方便安装

其他的问题 去 https://github.com/zdz/ServerStatus-Rust  大家给他点个 star!


我只修改了 tgbot 地址,方便国内服务器也能成功通知
还改了安装脚本 只在debian11 用



```bash
# 安装 服务端
cd ~ && wget https://raw.githubusercontent.com/jiuaiwo/ServerStatus/main/status.sh -O status.sh && bash status.sh -i -s

# 安装 客户端 配置采用了自动分组
cd ~ && wget https://raw.githubusercontent.com/jiuaiwo/ServerStatus/main/status.sh -O status.sh && bash status.sh -un -c && bash status.sh -i -c  grpc://cn:111111@你的服务端IP:9394
# grpc://cn:andyou@你的服务端IP:9394
# 上面地址 对应config.toml
# hosts_group = [
#   {gid = "cn", password = "111111", location = "🇨🇳", type = "kvm", notify = true},
#   {gid = "hk", password = "111111", location = "🇭🇰", type = "kvm", notify = true},
#   {gid = "home", password = "111111", location = "🏠", type = "kvm", notify = true},
#   # 例如不发送通知可以单独做一组
#   {gid = "silent", password = "111111", location = "🏡", type = "kvm", notify = false},
# ]
```

# 更多用法：
❯ bash status.sh

help:
    -i,--install    安装 Status
        -i -s           安装 Server
        -i -c           安装 Client
        -i -c conf      自动安装 Client
    -up,--upgrade   升级 Status
        -up -s          升级 Server
        -up -c          升级 Client
        -up -a          升级 Server和Client
    -un,--uninstall  卸载 Status
        -un -s           卸载 Server
        -un -c           卸载 Client
        -un -a           卸载 Server and Client
    -rc,--reconfig      更改 Status 配置
        -rc          更改 Client 配置
        -rc conf         自动更改 Client配置
    -s,--server     管理 Status 运行状态
        -s {status|start|stop|restart}
    -c,--client     管理 Client 运行状态
        -c {status|start|stop|restart}
    -b,--bakup      备份 Status
        -b -s          备份 Server
        -b -c          备份 Client
        -b -a          备份 Server and Client
    -rs,--restore    恢复 Status
        -rs -s          恢复 Server
        -rs -c          恢复 Client
        -rs -a          恢复 Server and Client
    -h,--help       查看帮助
若无法访问 Github: 
    CN=true bash status.sh args
```