# Filter Mediator

The **Filter Mediator** can be used for filtering messages based on an
XPath, JSONPath or a regular expression. If the test succeeds, the
Filter mediator executes the other mediators enclosed in the sequence.

The Filter Mediator closely resembles the "If-else" control structure.

---

## Info

The Filter mediator is a [conditionally content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

```java
<filter (source="[XPath|json-eval(JSONPath)]" regex="string") | xpath="[XPath|json-eval(JSONPath)]">
   mediator+
</filter>
```

This mediator could also be used to handle a scenario where two
different sequences are applied to messages that meet the filter
criteria and messages that do not meet the filter criteria.

```java
<filter (source="[XPath|json-eval(JSONPath)]" regex="string") | xpath="[XPath|json-eval(JSONPath)]">
   <then [sequence="string"]>
     mediator+
   </then>
   <else [sequence="string"]>
     mediator+
   </else>
</filter>
```

In this case, the Filter condition remains the same. The messages that
match the filter criteria will be mediated using the set of mediators
enclosed in the `then` element. The messages that do
not match the filter criteria will be mediated using the set of
mediators enclosed in the `else` element.
