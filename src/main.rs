fn main() {
    // code 3-1 型推論させた変数の使用例
    {
        let name = "masuda tomoaki";
        let age = 30;
    }

    // code 3-2 肩を宣言した変数の使用例
    {
        let name: &str = "masuda tomokai";
        let age: i32 = 30;
    }

    // code 3-3 正しい関数の宣言例
    {
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
    }

    // code 3-4 間違った関数の宣言例
    {
        // fn add(x: i32, y: i32) -> i64 {
        //     println!("call add");
        //     x + y
        // }
    }

    // code 3-5 浮動小数点数の使用例
    {
        let x = 100.234;
        println!("x is {}", x);
    }
    {
        let x: f64 = 100.234;
        println!("x is {}", x);
    }

    // code 3-6 論理値型の使用例
    {
        let f = true;
        println!("f is {}", f);
    }

    // code 3-7 文字型の使用例
    {
        let c = 'A';
        let c2 = 'あ';
        println!("c is {}", c);
        println!("c2 is {}", c2);
    }

    // code 3-8 println!の使用例
    {
        let s = "Hello Rust world.";
        println!("{}", s);
    }

    // code 3-9 複数のステークホルダーを指定したprintln!の使用例
    {
        let dog = "DOG";
        let cat = "CAT";
        println!("{} and {}", dog, cat);
    }

    // code 3-10 String型を使ったprintln!の使用例
    {
        let s = String::from("Hello Rust world.");
        println!("{}, ", s);
    }

    // code 3-11 「+」演算子を使った文字列の連結
    {
        let s1 = String::from("Hello");
        let s2 = String::from("Rust");
        let s3 = String::from("world.");
        let s = s1 + " " + &s2 + " " + &s3;
        println!("{}", s);

        /*
        &str は　 + 使えないっぽい??
        &は借用の印
        */
    }

    // code 3-12 format!マクロによる文字列の連結
    {
        let s1 = String::from("Hello");
        let s2 = String::from("Rust");
        let s3 = String::from("world.");
        let s4 = "&str";
        let num = 12;
        let s = format!("{} {} {} {} {}", s1, s2, s3, s4, num);
        println!("{}", s);

        /*
        format!マクロは数値や文字列を連結して一つの文字列(String)を返してくれる
        */
    }

    // code 3-13 「"」（ダブルクォート）で直接代入した文字列の連結
    {
        let s1 = "Hello";
        let s2 = "Rust";
        let s3 = "world.";
        let s = format!("{} {} {}", s1, s2, s3);
        println!("{}", s);
    }

    // code 3-14 タプルの使用例
    {
        let t = ("masuda", 30, 1);
        println!("name is {} age {} {}", t.0, t.2, t.3);
    }
}
