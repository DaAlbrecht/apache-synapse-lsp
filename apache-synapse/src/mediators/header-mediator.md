# Header Mediator

The **Header Mediator** allows you to manipulate SOAP and HTTP headers.

---

## Info

The Header mediator is a [conditionally content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<header name=”string” (value=”string|{property}” | expression=”xpath|jsonpath”) [scope=default|transport] [action=set|remove]/>
```

The optional `action` attribute specifies whether the
mediator should set or remove the header. If no value is specified, the
header is set by default.
