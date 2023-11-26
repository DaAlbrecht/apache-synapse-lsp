# XSLT Mediator

The **XSLT Mediator** applies a specified XSLT transformation to a
selected element of the current message payload. In addition, you can:

-   Specify properties already included in the mediation flow to be
    added to the XSLT script as XSLT parameters.
-   Specify features to be enabled/disabled in the XSLT transformation.
-   Import external XSLT scripts to the main XSLT script of the XSLT
    mediator by adding them as resources.

---

## Info

The XSLT mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
    <xslt key="string" [source="xpath"]>
         <property name="string" (value="literal" | expression="xpath")/>*
         <feature name="string" value="true| false" />*
         <resource location="string" key="string"/>*
    </xslt>
```
