## Linking from an store entry {#sec:thestore:linking}

In @sec:thestore:fileformat:header:imag was already defined that there MUST BE a
section "imag" in the header. This section can be used to link to "internal" and
"external" content, whereas "internal content" refers to entries which are
stored in the very same store as the entry which links.
The term "external content" means content which is not stored in the in the
store, but elsewhere on the filesystem or the network (thus, an URL is valid
external content).

Entries can be referenced from the content part. For example, if the content
part is written in Markdown, the user is able to link content within the
Markdown text.
These links could be either links to internal content or external content.

### Linking to internal content {#sec:thestore:linking:internal}

### Linking to external content {#sec:thestore:linking:external}

#### Weak linking {#sec:thestore:linking:external:weak}

#### Strong linking {#sec:thestore:linking:external:strong}

