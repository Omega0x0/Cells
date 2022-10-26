(Be careful, there's a lot of bad code out there./Будьте осторожны, там много плохого кода.)

# Cells

Contents/Содержание:
1. [[ENG]](#eng);
2. [[RUS]](#rus).

## [ENG]

Cells is an ordinary simulator of cells, namely their evolution, their indicators and abilities participate in the evolution, but they cannot create complex structures: connections with other cells, a single movement.

## Basic provisions:
1. A cell cannot divide into a place where there are already cells;
2. A cell cannot attack an empty space;
3. Each cell has a direction for attack and division;
4. The medium can nourish the cells located on it, but the amount of energy decreases with increasing height;
5. The energy consumption of cells is formed according to the following formula ``` DEFAULT_MIN_MASS / min_mass + max_mass / DEFAULT_MAX_MASS + time_life / max_time_life ```;
6. Attack and defense do not waste energy, but if the attack fails, the energy will be lost (``` resistance - damage ```);
7. Each cell has its own age, when the maximum set lifetime is reached, the cell dies, the same with the minimum mass;
8. Evolution occurs by pseudorandom mutations, with mutation a new species appears and all the parameters of the cell change;
9. Each cell has its own set of commands (genome), in which 3 types of commands can be prescribed: attack, change direction, multiply;
10. When threshold values are reached: genome size is zero, attack or defense is less than 0 - death.

## Interface:
1. World:
    * Lets you know the number of live cells;
    * Allows you to change the nutrient content of the medium;
    * Allows you to change the simulation speed.
2. Info:
    * Displays the average cell counts by adding up all the counts and dividing them by the total number of cells.
3. Filters:
    * Allows you to view the color display of the view indicators in relation to the average for the substrate.

---

## [RUS]

Cells - это обычный симулятор клеток, а именно их эволюции, в эволюции участвуют их показатели и способности, но они не могут создавать сложные структуры: связи с другими клетками, единое перемещение.

## Основные положения:
1. Клетка не может делиться в то место, где уже есть клетки;
2. Клетка не может атаковать пустое пространство;
3. Каждая клетка имеет направление для атаки и деления;
4. Среда может питать клетки находящейся на ней, но количество энергии падает с увеличением высоты;
5. Потребление энергии у клеток складывается по следующей формуле ``` DEFAULT_MIN_MASS / min_mass + max_mass / DEFAULT_MAX_MASS + time_life / max_time_life ```;
6. Атака и защита не тратят энергию, но если атака будет неудачной, то энергия будет потеряна (``` resistance - damage ```);
7. Каждая клетка имеет свой возраст, при достижении максимально установленного времени жизни, клетка умирает, то же самое и при минимальной массе;
8. Эволюция происходит путём псевдослучайных мутаций, при мутации появляется новый вид и все параметры клетки меняются;
9. Каждая клетка имеет свой набор команд (genome), в котором может быть прописано 3 вида команд: атака, изменить направление, размножиться;
10. При достижении пороговых значений: размер генома нулевой, атака или защита меньше 0 - смерть.

## Интерфейс:
1. World:
    * Позволяет узнать количество живых клеток;
    * Позволяет изменить питательность среды;
    * Даёт менять скорость симуляции.
2. Info:
    * Отображает средние показатели клеток, путём складывания всех показателей и деления их на общее количество кдеток.
3. Filters: 
    * Позволяет посмотреть цветное отображение показателей вида по отношению к средним по субстрату.
