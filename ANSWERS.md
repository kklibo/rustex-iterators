# Answers

<details>
<summary>ex01</summary>

```
range.collect()
```
</details>

<details>
<summary>ex02</summary>

```
numbers.iter().map(|x| x.is_power_of_two()).collect()
```
or
```
numbers.iter().copied().map(u8::is_power_of_two).collect()
```
</details>

<details>
<summary>ex03</summary>

```
numbers.iter().cycle().take(count).sum()
```
</details>

<details>
<summary>ex04</summary>

```
a.zip(b).filter(func).count()
```
</details>

<details>
<summary>ex05</summary>

```
values.iter().map(hash).reduce(xor)
```
</details>

<details>
<summary>ex06</summary>

```
values.iter().rev().enumerate().step_by(skip + 1).unzip()
```
</details>

<details>
<summary>ex07</summary>

```
strs.iter().flat_map(|x| x.chars()).find(|c| c.is_digit(16))
```
or
```
strs.iter().map(|x| x.chars()).flatten().find(|c| c.is_digit(16))
```
</details>

<details>
<summary>ex08</summary>

```
prefix.iter().chain(base).eq(with_prefix)
```
</details>


<details><summary>ex09</summary><blockquote>

<details>
<summary>func00</summary>
<code>
s.chars().all(char::is_numeric)
</code>
</details>

<details>
<summary>func01</summary>
<code>
s.chars().any(char::is_numeric)
</code>
</details>

<details>
<summary>func02</summary>
<code>
values.into_iter().flat_map(skip_first).collect()
</code>
</details>

<details>
<summary>func03</summary>
<code>
values.iter().filter_map(hexify).collect()
</code>
</details>

<details>
<summary>func04</summary>
<code>
values.iter().find_map(hexify)
</code>
</details>

</blockquote></details>


<details>
<summary>ex10</summary>

```
values.iter().scan(0, check_limit).partition(is_even)
```
</details>

<details>
<summary>ex11</summary>

```
strs.iter()
    .skip_while(|s| !is_start(s))
    .skip(1)
    .take_while(|s| !is_end(s))
    .try_fold(0, try_add)
```
</details>
