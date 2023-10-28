#### property

We repeat foaf:knows many times

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

#### rel

Similar to the example above

```html
<div vocab="http://xmlns.com/foaf/0.1/" resource="#me">
  <ul rel="knows">
    <li resource="http://example.com/bob/#me" typeof="Person">
      <a property="homepage" href="http://example.com/bob/"
        ><span property="name">Bob</span></a
      >
    </li>
    <li resource="http://example.com/eve/#me" typeof="Person">
      <a property="homepage" href="http://example.com/eve/"
        ><span property="name">Eve</span></a
      >
    </li>
    <li resource="http://example.com/manu/#me" typeof="Person">
      <a property="homepage" href="http://example.com/manu/"
        ><span property="name">Manu</span></a
      >
    </li>
  </ul>
</div>
```
