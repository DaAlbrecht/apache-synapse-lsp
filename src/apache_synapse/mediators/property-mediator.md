# Property Mediator

The **Property Mediator** has no direct impact on the message, but rather on the message context flowing through Synapse. You can retrieve
the properties set on a message later through the Synapse XPath Variables or the `get-property()` extension function. A property can have a defined scope for which it is valid. If a property has no defined scope, it defaults to the Synapse message context scope. Using the property element with the **action** specified as `remove`, you can remove any existing message context properties.

---

## Info

The Property mediator is a [conditionally content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

```java
<property name="string" [action=set|remove] [type="string"] (value="literal" | expression="xpath") [scope=default|transport|axis2|axis2-client] [pattern="regex" [group="integer"]]>
    <xml-element/>?
</property>
```

