//  Eu começarei tentando exercitar a memória muscular para saber exatamente o que é preciso. 
//  escrever naturalmente ao programar nessa linguagem.

//Biblioteca que permite a entrada e saida de valores IO dentro da biblioteca padrão.
use std::{cmp::Ordering, io};

//Biblioteca que Invoca numeros randomicos.
use rand::Rng;

//Função principal onde o codigo inicia quando execultado.
fn main ()  {
    
    //Função para escrever no terminal.
    println!("Adivinhe o número!\n");

    // Essa variável capita um numero de uma função que gera numeros aleatórios de 1 - 100
    let numero_escolhido = rand::thread_rng().gen_range(1..=100);

    //Essa linha é um laço de repetição, fará o programa rodar até uma resolução.
    loop { 
        println!("Digite o seu palpite.");

        
        //Variável mutável que pode receber imput.
        let mut palpite = String::new();

        
        //Função de entrada da Variável
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");
        
        //Essa função  converte a variável para um Inteiro. 
        //A função .trim() remover espaços e quebra de linhas para a conversão de inteiro.
        //A função .parse() faz a conversão da variável.
        let it_palpite: i32 = match palpite.trim().parse(){         //
            Ok(n) => n,                                        //
            Err(_) => {                                             //  Esse codigo funciona bem, só
                print!("Entrada invalida, tente novamente\n\n");    //  testando outras possibilidades.
                continue;                                           //
            }                                                       //
        };
        
        
        //Existem formas mais rapidas, mas essa parece ser uma das mais seguras. Até comparado com a função de pegar string acima
        /*if let Ok(n) = palpite.trim().parse::<i32>() {
            it_palpite = n;
        } else {
            print!("Entrada invalida, tente novamente");
            main();
            return;
        }*/

        //A função if é a mais classica e usada na programação, triste que com o tempo percebemos formas melhores que ela.
        /* 
        if it_palpite == numero_escolhido {
            print!("Parabens você acertou, o número era realmente: {}\n", it_palpite);

        } else{
            print!("Uma pena, você disse: {} ", it_palpite);
            print!("e o numero secreto era: {}\n", numero_escolhido);

        }*/

        // Essa função "Mach" faz uma comparação com maior variedade de opções, parece o switch case do java
        match it_palpite.cmp(&numero_escolhido){
            Ordering::Less => print!("Muito Baixo\n\n"),
            Ordering::Greater => print!("Muito Alto\n\n"),
            Ordering::Equal => {
                print!("Você acertou, parabens!\n");
                //Não gosto de usar o break, pode dar problema em codigos mais complexos, porem bora ver o que essa linguagem nos reserva...
                return;
            } 
        }
    }
}