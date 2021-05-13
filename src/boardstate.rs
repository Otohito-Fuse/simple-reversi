/// どちらのターンかを判定する列挙型
///
/// 駒などを判別するのにも使う。
/// 値を代入するときにムーヴだと面倒なのでCopyトレイトを実装。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    White,
    Black,
}

/// 盤面の情報を持つ構造体
///
/// sizeは盤面のサイズ（高さ=幅）。
/// stateは2次元配列で、各要素は```Option<Turn>```型
/// （```None```が駒が置かれていない状態、```Some(Turn::White）```が白い駒が置かれている状態、
/// ```Some(Turn::Black)```が黒い駒が置かれている状態）。
/// turnは今どっちのターンなのかの情報を持つ。
#[derive(Debug)]
pub struct BoardState {
    size: usize,
    state: Vec<Vec<Option<Turn>>>,
    turn: Turn,
}

impl BoardState {
    /// 新しい盤面を作成する
    pub fn new(n: usize, white_turn: bool) -> BoardState {
        assert!(n != 0);
        let mut s: Vec<Vec<Option<Turn>>> = vec![vec![None; 2 * n]; 2 * n];
        s[n - 1][n - 1] = Some(Turn::White);
        s[n - 1][n] = Some(Turn::Black);
        s[n][n - 1] = Some(Turn::Black);
        s[n][n] = Some(Turn::White);
        BoardState {
            size: 2 * n,
            state: s,
            turn: if white_turn { Turn::White } else { Turn::Black },
        }
    }

    /// 盤面の大きさを取得する
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// 盤面の状態をchar型の二次元配列で出力する
    pub fn show_board(&self) -> Vec<Vec<char>> {
        let n = self.size;
        let mut v: Vec<Vec<char>> = vec![vec![NO_PIECE; n]; n];
        for i in 0..n {
            for j in 0..n {
                if let Some(t) = &self.state[i][j] {
                    v[i][j] = match t {
                        Turn::Black => BLACK,
                        Turn::White => WHITE,
                    }
                }
            }
        }
        v
    }

    /// 白い駒（char型）
    pub fn white_piece() -> char {
        WHITE
    }

    /// 黒い駒（char型）
    pub fn black_piece() -> char {
        BLACK
    }

    /// どちらのターンかを駒の文字で出力
    pub fn which_turn(&self) -> char {
        match self.turn {
            Turn::Black => BLACK,
            Turn::White => WHITE,
        }
    }

    /// 白の番かどうか
    pub fn is_it_white_turn(&self) -> bool {
        self.turn == Turn::White
    }

    /// 駒の個数を出力
    pub fn count_pieces(&self) -> ((char, usize), (char, usize)) {
        let mut white_count: usize = 0;
        let mut black_count: usize = 0;
        let n = self.size;
        for i in 0..n {
            for j in 0..n {
                if let Some(t) = &self.state[i][j] {
                    match t {
                        Turn::White => white_count += 1,
                        Turn::Black => black_count += 1,
                    }
                }
            }
        }
        ((WHITE, white_count), (BLACK, black_count))
    }

    /// そこに置いたときに裏返せる駒の個数
    pub fn cnt_reversable(&self) -> Vec<Vec<usize>> {
        let n = self.size;
        let mut vec: Vec<Vec<usize>> = vec![vec![0; n]; n];
        let s = &self.state;
        for i in 0..n {
            for j in 0..n {
                if let Some(_) = s[i][j] {
                    // もう置いてあるマスはスルー
                    continue;
                }
                for k in 0..8 {
                    // 進む方向ごとに判定

                    // まず1マス隣
                    let new_x: i32 = i as i32 + dx(k);
                    let new_y: i32 = j as i32 + dy(k);

                    // 盤面から出ていた場合
                    if !BoardState::in_range(new_x, n) || !BoardState::in_range(new_y, n) {
                        continue;
                    }
                    let new_x: usize = new_x as usize;
                    let new_y: usize = new_y as usize;

                    // 隣のマスが空ならもう処理はいらない
                    if let Some(t) = s[new_x][new_y] {
                        // 隣のマスが自分と同じ色ならもう処理はいらない
                        if t == self.turn {
                            continue;
                        }
                        // 隣のマスが自分と違う色のときだけ進んで行く
                        for l in 1..n {
                            let new_x: i32 = new_x as i32 + l as i32 * dx(k);
                            let new_y: i32 = new_y as i32 + l as i32 * dy(k);

                            // 盤面から出たら終了
                            if !BoardState::in_range(new_x, n) || !BoardState::in_range(new_y, n) {
                                break;
                            }
                            let new_x: usize = new_x as usize;
                            let new_y: usize = new_y as usize;

                            // 空のマスに着いたら終了
                            if let None = s[new_x][new_y] {
                                break;
                            }

                            // 自分と同じ色が再び現れたらこのときだけ裏返せるので
                            // 裏返せる枚数をカウントアップ
                            if let Some(t) = s[new_x][new_y] {
                                if t == self.turn {
                                    vec[i][j] += l;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        vec
    }

    /// 置けるかどうかを判定
    fn puttable(&self) -> bool {
        let n = self.size;
        let vec = self.cnt_reversable();
        let mut flag: bool = false;
        for i in 0..n {
            for j in 0..n {
                if vec[i][j] > 0 {
                    flag = true;
                }
            }
        }
        flag
    }

    /// マス目に駒を置く操作
    ///
    /// 返り値は、ゲームを続けられる場合true、両者ともに置けるマスがない場合にfalse。
    pub fn put(&mut self, i: usize, j: usize) -> bool {
        let n = self.size;
        assert!(i < n && j < n);
        let vec = &self.cnt_reversable();
        assert!(vec[i][j] > 0);
        let s = &mut self.state;
        s[i][j] = Some(self.turn);
        for k in 0..8 {
            // 進む方向ごとに判定

            // まず1マス隣
            let new_x: i32 = i as i32 + dx(k);
            let new_y: i32 = j as i32 + dy(k);

            // 盤面から出ていた場合
            if !BoardState::in_range(new_x, n) || !BoardState::in_range(new_y, n) {
                continue;
            }
            let new_x: usize = new_x as usize;
            let new_y: usize = new_y as usize;

            // 隣のマスが空ならもう処理はいらない
            if let Some(t) = s[new_x][new_y] {
                // 隣のマスが自分と同じ色ならもう処理はいらない
                if t == self.turn {
                    continue;
                }
                // 隣のマスが自分と違う色のときだけ進んで行く
                for l in 1..n {
                    let new_x: i32 = new_x as i32 + l as i32 * dx(k);
                    let new_y: i32 = new_y as i32 + l as i32 * dy(k);

                    // 盤面から出たら終了
                    if !BoardState::in_range(new_x, n) || !BoardState::in_range(new_y, n) {
                        break;
                    }
                    let new_x: usize = new_x as usize;
                    let new_y: usize = new_y as usize;

                    // 空のマスに着いたら終了
                    if let None = s[new_x][new_y] {
                        break;
                    }

                    // 自分と同じ色が再び現れたらこのときだけ裏返せるので
                    // 実際に裏返していく
                    if let Some(t) = s[new_x][new_y] {
                        if t == self.turn {
                            // 間の駒を裏返していく処理
                            for m in 1..=l {
                                s[(i as i32 + m as i32 * dx(k)) as usize]
                                    [(j as i32 + m as i32 * dy(k)) as usize] = Some(self.turn);
                            }
                            break;
                        }
                    }
                }
            }
        }

        // ターンを交代
        self.turn = if self.turn == Turn::White {
            Turn::Black
        } else {
            Turn::White
        };

        // 置けるならtrueを返して終了
        if BoardState::puttable(&self) {
            return true;
        }

        // 置けないならもう一度ターンを交代
        self.turn = if self.turn == Turn::White {
            Turn::Black
        } else {
            Turn::White
        };

        // 今度は置けるならtrueを返す
        if BoardState::puttable(&self) {
            true
        } else {
            // 置けないならfalseを返す
            false
        }
    }

    /// マスの範囲内（0..n）かどうかを判定
    fn in_range(z: i32, n: usize) -> bool {
        z >= 0 && z < n as i32
    }
}

// これはダメっぽい
// const dx: Vec<isize> = vec![1,1,1,0,-1,-1,-1,0];
// const dy: Vec<isize> = vec![-1,0,1,1,1,0,-1,-1];

/// x方向への微小変化を見る用の配列の代わり
const fn dx(n: usize) -> i32 {
    match n {
        0 | 1 | 2 => 1,
        3 | 7 => 0,
        4 | 5 | 6 => -1,
        _ => 0,
    }
}

/// y方向への微小変化を見る用の配列の代わり
const fn dy(n: usize) -> i32 {
    match n {
        2 | 3 | 4 => 1,
        1 | 5 => 0,
        0 | 6 | 7 => -1,
        _ => 0,
    }
}

const WHITE: char = 'o';
const BLACK: char = '#';
const NO_PIECE: char = '.';
