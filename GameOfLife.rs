const TAM: int = 5;

fn clear_screen() {
    std::process::Command::new("sh").arg("-c").arg("clear").status().unwrap();
}

fn imprimir_matriz(matriz: &mut [[bool; TAM]; TAM]) {
    println!("  _0|_1|_2|_3|_4|_5|_6|_7|_8|_9|10|11|12|13|14|");
    for i in 0..TAM {
        print!("{} ", (i as u8 + 65) as char); // Letras das linhas

        for j in 0..TAM {
            if matriz[i][j] {
                print!("\u{2B1B}"); // Vivo imprime como quadrado preenchido
            } else {
                print!("__"); // Morto imprime como underline (quadrado vazio)
            }
            print!("|");
        }
        println!();
    }
}

fn main() {
    let mut matriz_atual: [[bool; TAM]; TAM] = [[false; TAM]; TAM];
    let mut matriz_futura: [[bool; TAM]; TAM] = [[false; TAM]; TAM];

    loop {
        clear_screen();
        println!("Jogo da vida de Conway\n");
        imprimir_matriz(matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Sair");
        println!("1 - Preencher manualmente a matriz");
        println!("2 - Usar um padrao pronto");
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: int = match opcao.trim().parse() 

        if opcao == 0 {
            break;
        }

    }
}

