use std::io; 

const TAM: usize = 15;

fn clear_screen() {
    std::process::Command::new("sh").arg("-c").arg("clear").status().unwrap();
}

fn imprimir_matriz(matriz: &[[bool; TAM]; TAM]) {
    println!("  _0|_1|_2|_3|_4|_5|_6|_7|_8|_9|10|11|12|13|14|");
    for i in 0..TAM {
        print!("{} ", (i as u8 + 65) as char); // Letras das linhas

        for j in 0..TAM {
            if matriz[i][j] {
                print!("\u{2588}\u{2588}"); // Vivo imprime como quadrado preenchido
            } else {
                print!("__"); // Morto imprime como underline (quadrado vazio)
            }
            print!("|");
        }
        println!();
    }
}

fn submenu_preenchimento_manual(matriz_atual: &mut [[bool; TAM]; TAM], matriz_futura: &mut [[bool; TAM]; TAM]) {
    loop {
        clear_screen();
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Voltar para o menu principal");
        println!("1 - Preencher uma celula");
        println!("2 - executar");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 {
            let mut caractere_y = String::new();
            let mut posY: i32;
            let mut posX: i32;

            posY = -1;
            while posY < 0 || posY > 14 {
                println!("Digite a coordenada Y (em letra maiúscula):");
                io::stdin().read_line(&mut caractere_y).expect("Falha ao ler a coordenada Y");
                let caractere_y: char = caractere_y.trim().chars().next().expect("Entrada inválida");

                posY = (caractere_y as i32) - 65; // Converte a letra para um índice inteiro
            }

            let mut pos_x = String::new();
            posX = -1;
            while posX < 0 || posX > 14 {
                println!("Digite a coordenada X:");
                io::stdin().read_line(&mut pos_x).expect("Falha ao ler a coordenada X");
                posX = pos_x.trim().parse().expect("Entrada inválida");
            }

            matriz_atual[posY as usize][posX as usize] = true; // Define a célula especificada como vivo

        }
    }
}

fn main() {
    let mut matriz_atual: [[bool; TAM]; TAM] = [[false; TAM]; TAM];
    let mut matriz_futura: [[bool; TAM]; TAM] = [[false; TAM]; TAM];

    matriz_atual[1][1] = true;

    loop {
        clear_screen();
        println!("Jogo da vida de Conway\n");
        imprimir_matriz(&matriz_atual);
        println!("\n\n\nEscolha uma opcao:");
        println!("0 - Sair");
        println!("1 - Preencher manualmente a matriz");
        println!("2 - Usar um padrao pronto");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");

        if opcao == 0 {
            break;
        } else if opcao == 1 {
            submenu_preenchimento_manual(&mut matriz_atual, &mut matriz_futura);
        }


    }
}

