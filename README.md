# A parser for lmatdocs

## Standard
```
**_bold and underlined_** (nested combinations with underscores inside asterisks)
*_italic and underlined_* (underscores inside asterisks)
***bold and italic*** (triple asterisks)
***_bold, underlined, and italic_*** (underlined nested)
# **Centered bold text** (center marker with formatting inside)
# *Centered italic text* (center marker with formatting inside)
# _Centered underlined text_ (center marker with formatting inside)
# **_Combined formatting in centered text_** (all combinations work in centered text)
```

## Notes
```
not asterisk nesting allowed (it's unreadable anyways)
***sometext** some text*

alternative:
1. ***some text*** *some text*
```

``` 
stack (Vec)
-------
begCenter
begUnderline
```

