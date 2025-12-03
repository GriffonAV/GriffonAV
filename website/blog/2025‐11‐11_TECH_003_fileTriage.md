---
slug: file_sorting
title: Sorting file by type
authors: rmabille
tags: [tech]
---

# Sorting Files

> report n°003 | 2025‐11‐11

## context

This research was done when faced with the following question:  
- Should we triage files depending on their types (file extension) to run specific yara rules on them ?

This question triggered two necessary research topics :
1. is there specific YARA rules for specific files format ?
2. if there is, is it interesting to use a triage method on those files ?

<!-- truncate -->

## quick answer

1. Yes there exist specific rules about specific files format as demonstrated by [this article describing a specific rule to detect certain pattern in a JPEG file](https://blog.didierstevens.com/2015/01/20/yara-rule-detecting-jpeg-exif-with-eval/)
2. As to is it interesting to triage files by type, yes and no. While is in the best interest for performance not to run rules meant for PDF on JPEG files, we should not forget that the extension or the header of a file can be easily overwritten in memory, so hiding a payload in a file by adding .xxx at the end of it to make it inconspicuous is a possibility.

## Targeted response

Now that we know that triage can be useful we need to determine in which case it really is. For that we need to differentiate between how malware operate, the category I want to expose is as I like to call them the "Weaponized documents".

Those type of malware are tempered documents such as PDF made to exploit a flaw in a specific software that reads those documents. As such they want to be opened as they are by the user, so transforming its type to hide itself wouldn't be effective.


## The advantages of doing a triage

### Different file types require different parsing logic

* YARA rules often depend on specific file structures (e.g., PE headers, PDF objects, Office streams).
* Applying the wrong rule to the wrong file type can cause false positives or rule failures.
* Some modules (like pe or elf) only work correctly when the file is of the expected type.
* Triage ensures rules are applied only where they make sense.

### Performance optimization

* Running all YARA rules on all files is slow and resource-heavy.
* File triage allows the engine to route only relevant rules to each file type.
* Reduces CPU and memory usage during scanning.
* Improves overall scan speed and reduces system impact.

### Allows preprocessing before YARA scanning

* Certain file types benefit from extraction or unpacking (e.g., unzip archives, unpack binaries, extract macros).
* Preprocessing reveals hidden or obfuscated malicious content.
* YARA scanning is more effective when scanning normalized or decompressed data.
* Increases detection accuracy by scanning the real payload rather than raw containers.