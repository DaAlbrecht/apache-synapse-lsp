# URLRewrite Mediator

The **URLRewrite Mediator** is used to modify and transform the URL
values available in messages. This can be done by defining a rewrite
action for each fragment of a selected property value. Alternatively,
you can rewrite the entire URL string at once.

---

## info

The URLRewrite mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

```java
<rewrite [inProperty="string"] [outProperty="string"]>
    <rewriterule>
        <condition>
        ...
        </condition>?
        <action [type="append|prepend|replace|remove|set"] [value="string"]
          [xpath="xpath"] [fragment="protocol|host|port|path|query|ref|user|full"] [regex="regex"]>+
    </rewriterule>+
</rewrite>
```
