#### resource & property

```html
<ul vocab="http://base.com/" resource="/alice/posts/trouble_with_bob">
  <li property="title">The trouble with Bob</li>
</ul>
```

```ttl
<https://rdfa.info/alice/posts/trouble_with_bob>
   <http://base.com/title> "The trouble with Bob" .
```

<hr>

#### combination of resource + property

```html
<ul vocab="http://base.com/" resource="/alice/posts/trouble_with_bob">
  <li resource="/alice/posts/trouble_with_bob" property="title">
    The trouble with Bob
  </li>
</ul>
```

```ttl
<https://rdfa.info/alice/posts/trouble_with_bob>
   <http://base.com/title> <https://rdfa.info/alice/posts/trouble_with_bob> .
```

<hr>

#### about

```html
<ul vocab="http://base.com/">
  <li about="/alice/posts/trouble_with_bob" property="title">
    The trouble with Bob
  </li>
</ul>
```

```ttl
<https://rdfa.info/alice/posts/trouble_with_bob>
   <http://base.com/title> "The trouble with Bob" .
```

<hr>

#### combination of about and resource

```html
<div about="/alice/posts/trouble_with_bob">
  <h2 property="title">The trouble with Bob</h2>
  <h3 property="creator" resource="#me">Alice</h3>
</div>
```

```ttl
<https://rdfa.info/alice/posts/trouble_with_bob>
   schema:title "The trouble with Bob";
   schema:creator <https://rdfa.info/play/#me> .
```

<hr>

#### more complex example

```html
<div vocab="http://xmlns.com/foaf/0.1/" resource="#me">
  <ul>
    <li property="knows" resource="http://example.com/bob/#me" typeof="Person">
      <a property="homepage" href="http://example.com/bob/"
        ><span property="name">Bob</span></a
      >
    </li>
    <li property="knows" resource="http://example.com/eve/#me" typeof="Person">
      <a property="homepage" href="http://example.com/eve/"
        ><span property="name">Eve</span></a
      >
    </li>
    <li property="knows" resource="http://example.com/manu/#me" typeof="Person">
      <a property="homepage" href="http://example.com/manu/"
        ><span property="name">Manu</span></a
      >
    </li>
  </ul>
</div>
```

```ttl
<https://rdfa.info/play/#me>
   foaf:knows <http://example.com/bob/#me>;
   foaf:knows <http://example.com/eve/#me>;
   foaf:knows <http://example.com/manu/#me> .
<http://example.com/bob/#me>
   rdf:type foaf:Person;
   foaf:homepage <http://example.com/bob/>;
   foaf:name "Bob" .
<http://example.com/eve/#me>
   rdf:type foaf:Person;
   foaf:homepage <http://example.com/eve/>;
   foaf:name "Eve" .
<http://example.com/manu/#me>
   rdf:type foaf:Person;
   foaf:homepage <http://example.com/manu/>;
```
