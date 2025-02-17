nix-commandとflakesが有効にする必要がある。

GitHubからコードを持ってくる場合
```sh
git clone https://github.com/pipopopipipopi/jisaku_2D_game.git
cd jisaku_2D_game
nix run
```

既にコードを持っている場合
flake.nixのあるディレクトリに入る。
```sh
nix run
```

nix-commandとflakesが有効になっていない場合は、
```sh
nix --experimental-features nix-command --experimental-features flakes run
```
で動作するかもしれないが、確証はない。
[参考](https://nixos.wiki/wiki/Nix_command)
