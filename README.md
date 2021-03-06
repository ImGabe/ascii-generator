# ascii-generator

> Transform images to ASCII

## Usage

```
Ascii-generator 0.1.0
Transform images to ASCII

USAGE:
    ascii-generator [FLAGS] [OPTIONS] <FILE> --size <size>

FLAGS:
    -h, --help       Prints help information
    -i, --invert     Invert file
    -V, --version    Prints version information

OPTIONS:
    -b, --bright <bright>    Output right [default: 0]
    -o, --output <output>    Output file
    -s, --size <size>        Output size

ARGS:
    <FILE>    File to process
```

## Example

```
$ ./ascii-generator -o ./example/image.txt -s 64 -i ./example/image.jpg

::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::;:;;;;;;;;;;;;
:::::::::::::::::::::::::::::::::::::::::::::::;;;;;;:;;;;;;;;;;
:::::::::::::::::::::::::::::::::::::::::::;;;;;;;;;;;;;;;;;;;;;
::::::::::::::::::::::::::::::::::::::::::;;;;;;;;;;;;;;;;;;;;;;
:::::::::::::::;;::;;;;;;;::::::;::::::::;;;;;;;;;;;;;;;;;;;;;;;
::::;:::;:;;;;:;;;;;;;;;;;;::::,:;::::::;;;;;;;;;;;;;;;;;;;;;;;;
::::;;:;;;;;;;;;;;;;;;;;;;;:::`l}/I;:::::;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;::''^+rLI;::::;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;:``'^!tL*I;::;;;;;;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;;;;;;;;:"```"~rQ#B;;;;;;;;;;;;;;;;;;;;;III;II
;;;;;;;;;;;;;;;;;;;;;;;;;;```````fxvcZ!;;;;;;;;;IIIII;IIIIIIIIII
;;;;;;;;;;;;;;;;;;;;;;;;;;I"^`^^^{)/YdMI;;;;;;IIIIIIIIIIIIIIIIII
;;;;;;;;;;;;;;;;;;;;;;I;;;-I:;II!{|xkkl;;;;;;IIIIIIIIIIIIIIIIIII
;;;;;;;;;;;;;;;I;II;IIII;I!!^^^^^(tkoiI;;;;IIIIIIIIIIIIIIIIIIIII
;;;;;;;IIIIIIIIIIIIIIIIIII;i",;I!|jh&I;;;;;IIIIIIIIIIIIIIIIIIIII
;IIIIIIIIIIIIIIIIIIIIIIIIIIi?/r-j|UOiI;;;;IIIIIIIIIIIIIIIIIIIIII
IIIIIIIIIIIIIIIIIIIIIIIIIIII_/l]c[zkI;;;;IIIIIIIIIIIIIIIIIIIIIII
IIIIIIIIIIIIIIIIIIIIIIIIIIII~r!?v]udI;;;IIIIIIIIIIIIIIIIIIIIIIII
IIIIIIIIIIIIIIIIIIIIIIIIIIII>u!?u?ndI;;;IIIIIIIIIIIlllllllIlllll
IIIIIIIIIIIIIIIIIIIIIIIIIIIIin!?x-rdI;;IIIIIIIllllllllllllllllll
IIIIIIIIIIIIIIIIIIIIIlllIIIIir!-r_rpI;IIIIIIIlllllllllllllllllll
IIIIIIIIIIIlllllllllllllllII>j!-r_rpII;IIIIIIlllllllllllllllllll
IIlIIlllllllllllllllllllllII<f!_j?xklIIIIIIIllllllllllllllllllll
lllllllllllllllllllllllllllI^```^xch>lIIIIIIllllllllll!!!l!l!!ll
lllllllllllllllllll!l!!!!lI;````^xcM8!I;;IIIIlllll!!!!!!!!!!!!!!
lllllllllllllll!!!!!!!!!!ll``^^^"rvUa<l;;;IIIllll!!!!!!!!!!!!!!!
llllllllll!!!!!!!!!!!!!!!l```^^^"fncQM>I;;;IIIlll!!!!!!!!!!!!!!!
!!ll!ll!!!!!!!!!!!!!!!!!l,``",!<-}/xUpZlI;;;IIlll!!!!!!!!!!!!!!!
!!!!!!!!!!!!!!!!!!!!!!!;^^^^^,Ii<-[(jvYUIlIIIIIll!!!!!!!!!!!!!!!
!!!!!!!!!!!!!!!!!!!!!!^`````^^,I!<_[)tnXC0!I;IIIll!!!!!!!!!!!!!!
!!!!!!!!!!!!!!!!iii!;^```````^^,;!~-[)fuYLZilIIIll!!!!!!!!iiiiii
!!!!!!!!!!!!!iiiii!"``````````^^:Ii~-}(jvJ0q!>IIIll!!!iiiiiiiiii
!!!!!!!ii!iiiiiiii"````````````^":l>+?{/xzCZd>!IIll!!!iiiiiiiiii
!!iiiiiiiiiiiiiii!^`````````````^,;!<_[)fvY0qh<!IIl!!!iiiiiiiiii
iiiiiiiiiiiiiiiii^``````````````^":l>+?{/xXLmb*illl!!iiiiiiii>i>
iiiiiiiiii>>>>>>"^``````````````^^,Ii~-}(jvJOdo{ill!!iiii>>>>>>>
ii>>i>>>>>>>>>>>^^````````'``````^";!<_])tnYQqa#>!l!!ii>>>>>>>>>
>>>>>>>>>>>>>>>,^^```````'''`````^":l>+?{/xzLwk*>i!!ii>>>>>>>>>>
```