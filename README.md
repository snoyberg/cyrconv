# СЧЯСОПЏ.ҒLЕХ
 
Счсопѵ.flех іѕ а fцппч lатіп2счгіllіс снагастег маррег. Вч dеfацlт іт марѕ lатіп снагастегѕ оf а gіѵеп техт то Счгіllіс (Lоок-Аlіке) соцптеграгтѕ, yвцт сап мар апч ѕет оf снагастегѕ цѕіпg сцѕтом flех fіlеѕ.

## Цѕаgе

### Gепегаl

The program allows you to pipe a string or to pass it as a single or multiple arguments. If you want to use the flex option you have to pass them at first argument.
There are currently six ways to use the program, as shown in the examples.

Ехамрlе:

```bash
    $ echo "Hello Friend!" | cyrconv
    $ echo "Hello Friends!" | cyrconv flex 1337-lite.flex
    $ cyrconv "Hello Friend!"
    $ cyrconv Hello my many Friends.
    $ cyrconv flex greek.flex "Hello Friends. How are you?"
    $ cyrconv flex greek.flex Hello my Friends. How are you?
```
### ' апd "

Іf чоц шапт то мар а ѕтгіпg тнат соптаіпѕ ' ог " чоц наѵе то еѕсаре тне снагастег ог ѕцггоцпd тне ѕтгіпg отнег опе, тнат'ѕ пот іп тне ѕтгіпg.

Ехамрlеѕ:

```bash
    $ echo "What's up" | cyrconv
    $ echo "That's pretty \"cheap\"." | cyrconv
    $ echo '"cheap" is what this is!' | cyrconv
    $ echo "If you want to map a string that contains \' or \" you have to escape the character \
    or surround the string other one, that\'s not in the string." | cyrconv
```

### Счгсопѵ.dеfацlт

Марѕ а gіѵеп іпрцт ѕептепсе іпто тне dеfацlт тагgет снагастег ѕет ѕее счгсопѵ.flех.

Тне fіlе dоеѕп'т пееd то ехіѕт.

> cyrconv sentence

Ехамрlе:

```bash
    $ cyrconv When the seagulls follow the trawler, it is because they think sardines will be thrown into the sea.
    Шнеп тне ѕеаgцllѕ fоllош тне тгашlег, іт іѕ весацѕе тнеч тніпк ѕагdіпеѕ шіll ве тнгошп іпто тне ѕеа.   
```

### Счгсопѵ.flех

Тне flех ортіоп аllошѕ чоц то lоаd сцѕтом тгапѕlатіоп fіlеѕ.

> cyrconv flex table_file sentence

Ехамрlе:

```bash
    $ cyrconv flex rot-13.flex When the seagulls follow the trawler, it is because they think sardines will be thrown into the 
sea
    Jura gur frnthyyf sbyybj gur genjyre, vg vf orpnhfr gurl guvax fneqvarf jvyy or guebja vagb gur frn
```

## Тне flех fіlе

Тне flех fіlе іѕ а ѕімрlе техт fіlе тнат сопѕіѕтѕ оf тшо lіпеѕ.

Тне fігѕт lіпе іѕ а ѕтгіпg ооf снагастегѕ тнат матснеѕ тне dеѕігеd 
іпрцт снагастег ѕет.

Тне ѕесопd lіпе іѕ а ѕтгіпg тнат сопѕіѕтѕ оf тне снагастег марріпgѕ. Вотн lіпеѕ ѕноцld соптаіп тне ѕаме пцмвег оf 
снагастегѕ.    
 
## Пісе то кпош

### Сігсцмflех

Втш. Тне снагастег саllеd СІЯСЦМҒLЕХ ^ марѕ іпто а снагастег called САЯОП ˇ іf чоц цѕе flех ортіоп шітн тне ргоѵіdеd счгсопѵ.flех 
fіlе. Тне снагастегs шоп'т ве сопѕіdегеd іп тне dеfацlт тгапѕlатіоп тавlе.

### Тне Яцѕѕіап 1337

Чоц сап аlѕо сомвіпе lеет-lіте шітн Счгіllіс 

```bash
    $ cyrconv flex 1337-lite.flex Hello my friend. Nastrowje. | cyrconv
    Н3ll0 мч fг13п). П4ѕтг0шј3.
```

or 

```bash
    $ cyrconv $(cyrconv flex 1337-lite.flex "Hello my friend. Nastrowje.")
    Н3ll0 мч fг13п). П4ѕт3г0шј3
```

### Іf тне Ѕоѵіет Цпіоп наd шоп тне Соld Шаг 

Ѕее fіlе "гцѕѕіап.соdе" 
