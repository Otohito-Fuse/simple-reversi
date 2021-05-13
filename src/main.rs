use std::time::Duration;
use std::thread::sleep;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod boardstate;
use boardstate::BoardState;

/// 整数の入力が不正である旨のメッセージ
fn err_not_int() {
    println!("半角数字で整数を入力して下さい．");
}

/// 入力が不適切な旨のメッセージ
fn err_input() {
    println!("入力が不適切です．");
}

/// 入力が範囲外の旨のメッセージ
fn err_not_range() {
    println!("入力が範囲外です．");
}

/// 盤面を表示させる
fn preview_board(bs: &BoardState) {
    let v = bs.show_board();
    let n = bs.get_size();
    print!("  ");
    for i in 1..=n {
        print!("{:2}", i);
    }
    println!("");
    for i in 0..n {
        print!("{:2}", i + 1);
        for j in 0..n {
            print!(" {}", v[i][j]);
        }
        println!("");
    }
}

/// 盤面を表示し，置けるマス目に+印をつける
fn preview_board_with_help(bs: &BoardState) {
    let v = bs.show_board();
    let cnt = bs.cnt_reversable();
    let n = bs.get_size();
    print!("  ");
    for i in 1..=n {
        print!("{:2}", i);
    }
    println!("");
    for i in 0..n {
        print!("{:2}", i + 1);
        for j in 0..n {
            print!(" {}", if cnt[i][j] > 0 { '+' } else { v[i][j] });
        }
        println!("");
    }
}

/// どちらのターンかを表示する
fn preview_turn(bs: &BoardState) {
    println!("{}のターン．", bs.which_turn());
}

/// 結果を表示する
fn show_result(bs: &BoardState) {
    let ((c1, s1), (c2, s2)) = bs.count_pieces();
    if s1 > s2 {
        println!("{0}が{1}個，{2}が{3}個で{0}の勝ち！", c1, s1, c2, s2);
    } else if s1 < s2 {
        println!("{0}が{1}個，{2}が{3}個で{2}の勝ち！", c1, s1, c2, s2);
    } else {
        println!("{0}が{1}個，{2}が{3}個で引き分け！", c1, s1, c2, s2);
    }
}

fn main() {
    println!("オセロをします．");

    // 盤面サイズの入力・決定
    let size: usize;
    loop {
        println!("盤面のサイズを4以上の偶数で入力してください．Returnキーで確定します．");
        let mut size_string = String::new();
        std::io::stdin().read_line(&mut size_string).ok();
        if let Ok(n) = size_string.trim().parse::<usize>() {
            if n >= 4 && n % 2 == 0 {
                size = n;
                break;
            } else {
                err_input();
            }
        } else {
            err_not_int();
        }
    }

    // CPUとやるかどうかの入力・決定
    let mut cpu_flag: bool = false;
    println!("CPUとやりますか？CPUとやる場合はy，そうではない場合はそれ以外を入力してください．");
    let mut y_or_no = String::new();
    std::io::stdin().read_line(&mut y_or_no).ok();
    if y_or_no.trim() == "y" {
        cpu_flag = true;
    }

    let mut i_am_white: bool = false;
    if cpu_flag {
        // どちらの番から始めるかの入力・決定
        loop {
            println!(
                "{0}として始める場合は1を，{1}として始める場合は2を入力してください．{0}が先攻です．",
                BoardState::black_piece(),
                BoardState::white_piece()
            );
            let mut size_string = String::new();
            std::io::stdin().read_line(&mut size_string).ok();
            if let Ok(n) = size_string.trim().parse::<usize>() {
                match n {
                    1 => {
                        break;
                    }
                    2 => {
                        i_am_white = true;
                        break;
                    }
                    _ => {
                        err_not_range();
                    }
                }
            } else {
                err_not_int();
            }
        }
    }

    // 盤面作成
    let mut bs = BoardState::new(size / 2, false);

    // ヘルプ（+印）を表示するかどうか
    let mut with_help_or_not: bool = false;

    // ゲーム実行
    loop {
        // 盤面の表示
        if with_help_or_not {
            preview_board_with_help(&bs);
        } else {
            preview_board(&bs);
        }

        // どちらのターンかの表示
        preview_turn(&bs);

        // CPUの番の場合
        if cpu_flag && (i_am_white || bs.is_it_white_turn()) && !(i_am_white && bs.is_it_white_turn()) {
            // 乱数発生用
            let mut rng = thread_rng();

            // 時間を空けつつメッセージを表示
            sleep(Duration::from_millis(250));
            println!("\nCPU操作中...\n");
            sleep(Duration::from_millis(750));

            // 置けるマス目を重み付けしつつVecで管理
            let mut options: Vec<(usize,usize)> = Vec::new();
            let vec = &bs.cnt_reversable();
            let n = bs.get_size();
            for i in 0..n {
                for j in 0..n {
                    if vec[i][j] > 0 {
                        for _ in 0..vec[i][j] {
                            options.push((i,j));
                        }
                    }
                }
            }

            // ランダムに選ぶ
            let &(i,j) = options.choose(&mut rng).unwrap();

            // マス目更新
            let can_continue = bs.put(i,j);

            // 続行できないときはループを抜けてゲームを終了
            if !can_continue {
                break;
            }
            continue;
        }

        // 以下、自分の番の場合

        // 操作方法の表示
        println!(
            "駒を置く場所を行番号，列番号の順で指定して下さい．Return区切りで入力してください．"
        );
        println!("もうゲームを終わって結果を見たい場合は1つ目の数字として0を入力していください．");
        if !with_help_or_not {
            println!(
                "駒が置ける場所のヒントを見たい場合は1つ目の数字として{}を入力してください．",
                size + 1
            );
        } else {
            println!("");
        }

        // 1つ目の数字受け取り
        let row_num: usize;
        loop {
            let mut row_num_string = String::new();
            std::io::stdin().read_line(&mut row_num_string).ok();
            if let Ok(n) = row_num_string.trim().parse::<usize>() {
                if n < size + 1 || (n == size + 1 && !with_help_or_not) {
                    row_num = n;
                    break;
                } else {
                    err_not_range();
                }
            } else {
                err_not_int();
            }
        }

        // 終了処理
        if row_num == 0 {
            println!("本当に終了しますか？はいならy，いいえならそれ以外を入力してください．");
            let mut y_or_no = String::new();
            std::io::stdin().read_line(&mut y_or_no).ok();
            if y_or_no.trim() == "y" {
                break;
            } else {
                continue;
            }
        }

        // ヘルプ表示処理
        if row_num == size + 1 {
            with_help_or_not = true;
            continue;
        } else {
            with_help_or_not = false;
        }

        // 2つ目の数字受け取り
        let column_num: usize;
        loop {
            let mut column_num_string = String::new();
            std::io::stdin().read_line(&mut column_num_string).ok();
            if let Ok(n) = column_num_string.trim().parse::<usize>() {
                if n > 0 && n <= size {
                    column_num = n;
                    break;
                } else {
                    err_not_range();
                }
            } else {
                err_not_int();
            }
        }

        // 置けるマス目かどうか判定
        let v = bs.cnt_reversable();
        if v[row_num - 1][column_num - 1] == 0 {
            println!("そこには置けません．");
            continue;
        }

        // マス目更新
        let can_continue = bs.put(row_num - 1, column_num - 1);

        // 続行できないときはループを抜けてゲームを終了
        if !can_continue {
            break;
        }
    }

    // 盤面表示
    preview_board(&bs);
    // 結果表示
    show_result(&bs);
}
