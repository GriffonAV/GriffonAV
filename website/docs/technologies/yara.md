# YARA ( Yet Another Ridiculous Acronym )

YARA **RULES** are logical expressions used to classify and identify malware samples by defining specific textual or binary malware detection patterns.
They are use in different context but were mainly developed for malware researcher and detection.

They were created by Victor Álvarez when he was working at VirusTotal a google subsidiary which specialize in malware research and detection. Over time, VirusTotal adopted and integrated YARA deeply into its own malware scanning and rule-testing infrastructure. And eventually, the tools they created to use YARA rules became open source and was embraced widely by the security community.

so YARA **the open source software** is a library that you can use to pattern matched rules with data. It is written in C and has a variety of API to handle C++ and other language implementation.

# What we use

At the start we were planning to use the c yara library and integrate it in our rust code, but integrating a c library in rust cause more complexity and lose a bit of performance.  
So we decided to integrate **[yara-x](https://github.com/VirusTotal/yara-x)** instead, which is a official rust rewrite of the original c yara library. ( official as in done by the current maintainers of the c yara library)

# Technical details

## YARA rules structure

YARA rules are composed of 3 sections:
- `strings` string or byte array defining the pattern to look out for.
- `conditions` a set of conditions to define the behavior of the rule.
- `metadata` diverse information about the rule.

### Strings

The main component are the strings, they can be plain ascii string or byte arrays they are defined like a variable in php `$x = ...`.

example:  
```
strings:
    $s1 = "example string"
    $s2 = 02 24 58 64 28 85 95
```

Those strings are patterns that you can find in the malicious code you are trying to find.

### Conditions


### Metadata

Information in the metadata field depends on what standard the one who created the rule follows, the following are the most common metadata you can find on almost every rule:
* `description` information about the purpose of the rule (sometimes just the name of the malware it was made for).
* `date` denotes the creation date of the rule and should be in the format YYYY-MM-DD.
* `reference` link to a report, source code repository, website, private report name, identifier, or a short description of the source from which the rule was derived. Can also serve to reference the malware.
* `author` author, group, or organization that composed or released the rule.  


The name of the rule is also an important information, as it is oftentimes the first and only information the user see about the rule. As such it is a good practice to include information in the form of tags in the name of the rule. For example you could use _MacOS to detail that the malware operates on mac.  

A list of the most common tags and good practices can be found here: [yara-style-guide](https://github.com/Neo23x0/YARA-Style-Guide)

## How we scan your computer

## where are we getting our rules ?

As writing yara rules takes a lot of knowledge in binary patterns and malware in general, we as an application not entirely focused on malware detection, leave writing these rules to actual malware researchers.

the majority of our rules are taken from those sources:
* link 1
* link 2
* link 3



