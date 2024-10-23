# dervy

In domain-driven design, entity types should be compared by identity rather than value.
dervy allows you to annotate your domain entities in order to derive
implementations of PartialEq, Eq, and Hash that only consider identity for equality.