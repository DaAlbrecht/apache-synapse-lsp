# Switch Mediator

The **Switch Mediator** is an XPath or JSONPath filter. The XPath or JSONPath is evaluated and returns a string. This string is matched against the regular expression in each switch case mediator, in the specified order. If a matching case is found, it will be executed, and the remaining switch case mediators are not processed. If none of the case statements are matching, and a default case is specified, the default will be executed.

---

## Info

The Switch mediator is a  [Conditionally Content-Aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
    <switch source="[XPath|json-eval(JSON Path)]">
       <case regex="string">
         mediator+
       </case>+
       <default>
         mediator+
       </default>?
    </switch>
```

