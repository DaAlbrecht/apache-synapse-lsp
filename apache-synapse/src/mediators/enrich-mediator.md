# Enrich Mediator

The **Enrich Mediator** can process a message based on a given source configuration and then perform the specified action on the message by using the target configuration. It gets an `OMElement` using the configuration specified in the source and then modifies the message by putting it on the current message using the configuration in the target.

---

## Info

The Enrich mediator is a [content-aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<enrich>
    <source [clone=true|false] [type=custom|envelope|body|property|inline] xpath | json-eval(JSON-Path)="" property="" />
    <target [action=replace|child|sibiling] [type=custom|envelope|body|property|inline|key] xpath | json-eval(JSON-Path)="" property="" />
</enrich>
```
