## NOTE:
1. THIS CRATE IS NOT PRODUCTION READY YET (Use at your own risk)
2. Contributions are welcome, but I am not actively accepting any at the moment, unless it's really crucial, Sorry
3. I am very busy at the moment, so finishing this up might take sometime, but I definitely have plans to do that.
4. This isn't a feature complete implementation as I only added some of the features, I need.



### Future plans
1. Add `prefix` to get all valid words extending from a specific prefix e.g given `BAT`, we can return `BAT`, `BATHE`, `BATHS`, `BATHROOM` e.t.c 
2. Add `anagrams`: to get all possible valid words that can be formed using only the letters in the provided argument e.g given `silent`, we should can 
return `vec!["listen", "enlist", "silent", "inlets"]` e.t.c 

## REFERENCES
1. [Incremental Construction of Minimal Acyclic Finite-State Automata](https://aclanthology.org/J00-1002.pdf)
2. [Compressing Dictionaries with a DAWG](http://stevehanov.ca/blog/?id=115)



