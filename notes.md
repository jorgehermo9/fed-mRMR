# sparse vs dense

## Aprox4.csv.disc (10 bins)

Dataset con 122 características y 10 valores por característica. un 90% de las posiciones
del bitmap tendrá valor 0.

Multiplicación de matrices sparse, más rápida que la dense?

### Matrix construction
Dense -> 8.2s 
Sparse -> 2.62s

## Connect4
### Matrix construction
Dense -> 1.35
sparse -> 1.71

## nci9
Dataset con ~10000 características. ~3 valores por característica ->~30000 características.
Un 33% de las posiciones del bitmap tendrá valor 0. Adecuado para sparse pero podría ser mejor

### Matrix construction
Dense -> 116.75s
Sparse -> 176.35s
### Mrmr calculation
Dense -> 181.79s
Sparse -> 341.87s

## Letter recognition

### Matrix construction
Dense -> 1.34s
Sparse -> 0.40s

## credit card (10 bins)

### Matrix construction
Dense -> 15.12s
Sparse -> 12.07s

# Info

Hacer multiplicación en sparse

El resultado guardarlo en uno dense. La mayoría de intersecciones no serán 0, es mejor
guardarlo como dense. 

La representación sparse no benefica tanto. Depende de los valores que haya por cada categoría.
Podría ser beneficioso utilizar microarrays, pero 
