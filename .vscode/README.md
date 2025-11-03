# VS Code スニペット 使用方法

このディレクトリには、AtCoderで使用するRustのコードスニペットが含まれています。
以下のプレフィックスを入力することで、対応するコードを展開できます。

## 目次

### 🌳 遅延評価セグメント木 (LazySegtree)

| プレフィックス | 説明 |
|---|---|
| `lazy_affine` | 遅延評価セグメント木（アフィン変換・区間和） |
| `lazy_add_sum` | 遅延評価セグメント木（区間加算・区間和） |
| `lazy_update_min` | 遅延評価セグメント木（区間代入・区間最小値） |
| `use_lazy` | 遅延評価セグメント木の使用（汎用） |

### 🌲 セグメント木 (Segtree)

| プレフィックス | 説明 |
|---|---|
| `seg_sum` | セグメント木（区間和） |
| `seg_max` | セグメント木（区間最大値） |
| `seg_min` | セグメント木（区間最小値） |
| `seg_from` | セグメント木（Vecから作成） |
| `use_seg` | セグメント木の使用（汎用） |

### 🧱 モノイド定義

| プレフィックス | 説明 |
|---|---|
| `def_monoid` | Monoid トレイトの定義 |
| `def_mapmonoid` | MapMonoid トレイトの定義 |

## 使用方法

1. Rustファイル（`.rs`）を開く
2. 上記のプレフィックスを入力
3. IntelliSenseの候補から選択、または`Tab`キーで展開
4. `Tab`キーで次のプレースホルダーに移動しながら、必要な値を入力

## 詳細

### 遅延評価セグメント木 (LazySegtree)

#### `lazy_affine` - アフィン変換・区間和
区間に対してアフィン変換 `f(x) = ax + b` を適用し、区間和を取得できます。
- 区間更新: `segtree.apply_range(l..r, (a, b))`
- 区間和取得: `segtree.prod(l..r).0`

#### `lazy_add_sum` - 区間加算・区間和
区間に対して値を加算し、区間和を取得できます。
- 区間加算: `segtree.apply_range(l..r, x)`
- 区間和取得: `segtree.prod(l..r).0`

#### `lazy_update_min` - 区間代入・区間最小値
区間に対して値を代入し、区間最小値を取得できます。
- 区間代入: `segtree.apply_range(l..r, Some(x))`
- 区間最小値取得: `segtree.prod(l..r)`

#### `use_lazy` - 汎用使用
既に定義されたMapMonoidを使用して遅延評価セグメント木を利用します。

### セグメント木 (Segtree)

#### `seg_sum` - 区間和
区間和を計算するセグメント木です。
- 値の設定: `seg.set(i, x)`
- 区間和: `seg.prod(l..r)`

#### `seg_max` - 区間最大値
区間最大値を計算するセグメント木です。
- 値の設定: `seg.set(i, x)`
- 区間最大値: `seg.prod(l..r)`

#### `seg_min` - 区間最小値
区間最小値を計算するセグメント木です。
- 値の設定: `seg.set(i, x)`
- 区間最小値: `seg.prod(l..r)`

#### `seg_from` - Vecから作成
既存のVecから直接セグメント木を作成します。

#### `use_seg` - 汎用使用
既に定義されたMonoidを使用してセグメント木を利用します。

### モノイド定義

#### `def_monoid` - Monoid トレイトの定義
カスタムモノイドを定義するためのテンプレートです。
- `identity()`: 単位元を返す
- `binary_operation()`: 二項演算を定義

#### `def_mapmonoid` - MapMonoid トレイトの定義
カスタムMapMonoidを定義するためのテンプレートです。
- `identity_map()`: 恒等写像を返す
- `mapping()`: 作用素を要素に適用
- `composition()`: 作用素の合成
