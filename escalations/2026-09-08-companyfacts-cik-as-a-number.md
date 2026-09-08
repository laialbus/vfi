# Escalation: a companyfacts document that spells its cik as a number

- Date: 2026-09-08
- Branch: M4-20

`company_facts` retrieves nothing for the great majority of filers, and the
golden fixtures do not see it.

EDGAR publishes the `cik` of a company facts document two ways. Some documents
carry the ten-digit string the submissions endpoint uses — `"cik":"0002003750"`
— and most carry a bare number, `"cik":1753391`. `CompanyFacts` in
`crates/fetch/src/edgar/documents.rs` types that field as a string, so a
document of the second kind is refused whole:

    not the document this publishes, because invalid type: integer `1753391`,
    expected a string at line 1 column 14

I sampled 217 filers while looking for this task's fixture, and 207 of them are
the second kind. Among them are CIK 0000320193 (Apple) and CIK 0001739104
(Elanco), whose filing history is already the fixture
`a-history-longer-than-its-first-page` — so the repository holds a filer whose
history retrieves and whose facts do not.

Nothing catches this today. Both facts fixtures are recordings of filers of the
first kind, `every-fact-a-filer-reported` by chance and this branch's
`a-filer-that-changed-its-fiscal-year` by choice — I filtered candidates on the
spelling once I found the refusal, because the alternative was a fixture that
pins a parse failure. The gate is green over a retrieval that works for about
one filer in twenty.

M4-20 is delivered and nothing is blocked. I did not fix this: the task says
the stage is unchanged, and one run is one task.

To resolve: a task that makes the retrieval read both spellings. It carries a
decision that is not the implementing run's, which is why this is here rather
than left to whoever claims it — `Filer.cik` is a contract field, and what
crosses when the document publishes `1753391` is either those characters or the
padded `0001753391` the request named. Normalize will key a filer by that
string, so the two spellings cannot both cross. Deciding it settles whether the
fetch to normalize contract needs saying that the key is padded, and a fixture
recorded from a filer of the second kind is what would then pin it.
