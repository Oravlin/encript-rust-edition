use std::{str::{self, FromStr}};

use rand::random_range;

const ALFABETO_PADRAO:&str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890áäàãâéëèêíïîìóöôõòúûüùÁÄÀÂÃÉËÈÊÜÚÙÛÍÏÎÌÓÖÕÒÔÚÜÛÙ[]{}()/|'~^&*-+!@#$%=?";

// criando função de entrada por terminal
fn input<T: FromStr>() -> Result<T, T::Err>{
    let mut input :String = String::new();
    std::io::stdin().read_line(&mut input).expect("ixi");
    return input.trim().parse::<T>()
}

fn encriptar(vetor_padrao:&Vec<&str>, alfabeto_alterado:&Vec<&str>) -> String{
    println!("O que deseja encriptar?");
    let texto :String  = input().expect("eita");
    // transforma o texto em um vetor
    let mut texto_vec :Vec<&str> = texto.split("").collect();
    // retira valores vazios
    texto_vec.remove(0);
    texto_vec.remove(texto_vec.len() - 1);
    let mut resposta :String = String::new();

    // converte as letras comuns nas letras encriptadas
    for letra in &texto_vec{
        // pega o index
        let letra_atual_index = vetor_padrao.iter().position(|x| x == letra).unwrap();
        // troca a letra
        let letra_trocada = alfabeto_alterado[letra_atual_index];
        // adiciona a letra para a variável final
        resposta.push_str(letra_trocada);
    }
    return resposta
}

fn decriptar(vetor_padrao:&Vec<&str>, alfabeto_alterado:&Vec<&str>) -> String{
    println!("O que deseja decriptar?");
    let texto :String = input().expect("eita");
    let mut texto_vec :Vec<&str> = texto.split("").collect();
    texto_vec.remove(0);
    texto_vec.remove(texto_vec.len() - 1);
    let mut reposta :String = String::new();

    for letra in &texto_vec{
        let letra_trocada_index = alfabeto_alterado.iter().position(|x| x == letra).unwrap();
        let letra_original = vetor_padrao[letra_trocada_index];
        reposta.push_str(letra_original);
    }
    return reposta
}

fn main() {
    // transforma a string padrão em um vetor, que permite uma manipulação melhor
    let mut vetor_padrao:Vec<&str> = ALFABETO_PADRAO.split("").collect();
    
    // pela identificação ser uma string vazia, ele pega um valor vazio no primeiro e último elemento. Essas duas função abaixo apagam elas
    vetor_padrao.remove(0);
    vetor_padrao.remove(vetor_padrao.len() - 1);

    // se fosse usado let mut alfabeto_alterado = vetor_padrao daria um erro de borrow, fazendo que os valores mudem em alfabeto_alterado mudassem em vetor_padrao
    // usando o sistema de referencia (let alfabeto_alterado = &mut vetor_padrao) daria erro mais tarde na exibição
    let mut alfabeto_alterado = vetor_padrao.clone();

    //utilizando fisher-yates para aleatorizar o alfabeto_alterado (tive que usar a crate rand pra fazer isso)
    // rev altera a ordem. usar alfabeto_alterado.len()..0 dá erro, pois o primeiro valor é sempre o inicio da contagem
    for i in (0..alfabeto_alterado.len()).rev(){
        // pega um número aleatório de 0 até o i, para cumprir que 0 <= j <= i fazendo que o algoritmo funcione
        let j = random_range(0..=i);
        
        // função nativa que troca os valores de lugar de uma forma segura
        alfabeto_alterado.swap(i, j);
    }

    loop{
        println!("Digite o comando (digite 'parar' ou 'p' para parar o programa) :");
        let comando:String = input().expect("deu ruim ao dar comando bro");
        match comando.to_ascii_lowercase().as_str(){
           "encript" | "encrip" | "e" => println!("{}", encriptar(&vetor_padrao, &alfabeto_alterado)),
           "decript" | "decrip" | "d" => println!("{}", decriptar(&vetor_padrao, &alfabeto_alterado)),

           "malfabeto" => println!("{:?} \n{:?}", &vetor_padrao, &alfabeto_alterado),

           "parar" | "stop" | "st" | "p"=> {
                println!("Parando programa...");
                break;
            },
           _ => {
            println!("Conheço esse comando não browski");
            continue;
            }
        };
    }
}
