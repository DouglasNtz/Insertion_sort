# Insertion_sort

OBS: Ao contrário do clássico insertion sort que é c * n * n, meu insertion sort é c * n * log2(n), se equiparando ao merge sort.

Algoritmos de ordenação de vetor de números escritos em Rust

Para executar um dos 4 algoritmos, basta digitar o seguinte comando:

cargo run numero_experimentos tamanho_vetor metodo_de_ordenacao tipo_output >> nome_arquivo.txt

- numero_experimento é um número inteior que representa a quantidade de vezes que geraremos um vetor aleatório e faremos sua ordenação in-place. Faremos parse dessa String para usize.

- tamanho_vetor é um número inteiro que representa a quantidade números aleatórios que serão gerados e armazenados no vetor, o qual faremos sua ordenação in-place. Deve ser possível fazer parse para usize.

Em metodo_de_ordenacao devemos digitar um dentre os quatro metodos disponíveis: 

- my_insertion_sort: o mais otimizado dos quatro métodos. Implementei com minha criatividade uma versão melhorada do insertion_sort
- insertion_sort: algorimo simples no qual vamos retirando da direita para esquerda os números no vetor v (alvo da ordenação) e vamos inserindo nm outro vetor de modo ordenado. Depois atribuímos esse vetor resultante ao vetor algo da ordenação.
- my_swap_sort: segundo metodo mais rápido dos quatro métodos. Implementei com minha criatividade uma versão melhorada do swap_sort
- swap_sort: algorimo simples no qual na etapa n, pegamos o valor da posição n+1 e testamos se é menor do que o valor imediatamente a sua esquerda. Se for, fazemos swap dos dois valores e continuamos o processo. Caso não seja, pulamos para o
  valor da posição n+2, até iterarmos por todo o vetor.

Em tipo_output devemos digitar uma das seguintes opções:

- Times: mostra o tempo total de todas as execuções (a soma dos tempos de cada execução).
- Iterations: mostra a média do número simplificado de iterações por execução, no sentido em que cada looping registra uma iteração.

Exemplos:

  cargo run --release 100 5000 my_insertion_sort Times >> resultados.txt

  Escreve o seguinte output no arquivo:

//---------------------------

  Function: my_insertion_sort

  Número de experimentos: 100

  Tamanho da lista de números: 5000

  Tempo total: 41.191591ms

//--------------------------

  cargo run --release 100 5000 my_insertion_sort Iterations >> resultados.txt

  Escreve o seguinte output no arquivo:

//--------------------------

  Function: my_insertion_sort

  Número de experimentos: 100

  Tamanho da lista de números: 5000
  
  Iterações por execução: 52935.43

  
