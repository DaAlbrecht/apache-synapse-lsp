# Validate Mediator

You can use the Validate mediator to validate XML and JSON messages.

---

## Info
The Validate mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Validating XML messages

The Validate mediator validates XML messages against a specified schema.
You can specify an XPath to extract and validate a specific part of the
message. Otherwise, the mediator validates the first child of the SOAP
body of the current message.

A [Fault mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/fault-mediator/) should be added as a child to the Validate mediator in order specify the fault sequence to be followed if the validation fails.


## Syntax

``` java
<validate [source="xpath"]>
   <property name="validation-feature-id" value="true|false"/>*
   <schema key="string"/>+
   <on-fail>
      mediator+
   </on-fail>
</validate>
```

