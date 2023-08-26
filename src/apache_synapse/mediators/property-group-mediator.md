# Property Group Mediator

The Property Group Mediator is similar to the [Property Mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/property-mediator/). It sets or removes properties on the message context flowing through synapse. However, unlike the Property mediator, the Property Group mediator handles multiple properties as a
group. You can select the property action (i.e., whether the property
must be added to or removed from the message context) for each
individual property. Therefore, in a scenario where you need to
set/remove multiple properties, you can add a single Property Group
Mediator configuration instead of multiple Property Mediator
configurations.

---

## Info

The Property Group mediator is a [conditionally content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

```java
<propertyGroup>
    <property name="name0" value="value0"/>
    <property name="name1" value="value1"/>
    <property name="name2" value="value2"/>
    ........
</propertyGroup>
```

