# Sequence Mediator

The **Sequence Mediator** refers to an already defined sequence element,
which is used to invoke a named sequence of mediators. This is useful
when you need to use a particular set on mediators in a given order
repeatedly.

You can alternatively select a predefined sequence from the Registry as
the in/out/fault sequence for a proxy service or a REST service without
adding any mediator configurations inline. The difference between these
two options are described in the table below.

| Attribute                                         | Picking a predefined sequence as in/out/fault sequence                                                                                                                                                                                                                  | Referring to a predefined sequence via the Sequence mediator                                                                                                                                                                                                        |
|---------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Adding other mediators                            | Other mediator configurations that are not already included in the predefined sequence cannot be added to the in/out/fault sequence.                                                                                                                                    | Other mediator configurations that are not already included in the predefined sequence can be added to the in/out/fault sequence                                                                                                                                    |
| Applying changes done to the predefined sequence | Any changes done to the sequence saved in the **Registry** after it was selected as the in/out/fault sequence will not be considered when carrying out mediation. | Any changes done to the sequence saved in the **Registry** after it was selected as the in/out/fault sequence will be considered when carrying out mediation. |

---

## Info

The Sequence mediator is a [content-unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

A sequence ref token refers to a `<sequence\>`
element, which is used to invoke a named sequence of mediators.

``` java
<sequence key="name"/>
```
