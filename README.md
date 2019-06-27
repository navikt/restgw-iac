[![CircleCI](https://circleci.com/gh/navikt/restgw-iac/tree/master.svg?style=svg)](https://circleci.com/gh/navikt/restgw-iac/tree/master)

# restgw-iac
Setter opp fasit-ressurser for applikasjoner i FSS og konsumenter (proxy) i SBS, og kobler dem sammen med en konsument gjennom rest-gateway. For `teamsykefravr`.

For å lage en kobling mellom applikajson og konsument legger du appene til i listen i `./configuration.json` slik:

```json
{
  "application_name": "<navn_på_applikajson>",
  "consumer_name": "<navn_på_konsument>"
}
```

Med mindre ressursene allerede finnes i fasit vil dette opprette:
- en applikasjon i fasit for applikasjonen og konsumenten,
- en application instance for applikasjonen og konsumenten i q1 og p,
- en RestService fasitressurs koblet til application instancene til applikasjonen og konsumenten.

Etter disse ressursene er opprettet vil applikasjonen og kosnumenten kobles sammen i rest-gateway i q1 og p. Først registerers applikasjonen som eksponert i q1 og p med `teamsykefravr` som eier. Derretter registreres konsumenten som konsument av RestServicen applikajsonen eksponerer. Til slutt oppdaterers rest-gateway med den nye koblingen.

Etter jobben har blitt kjørt kan man finne rest-gateway ressursene på fasit. En gateway url som er urlen som konsumenten skal bruke for å snakke med applikasjonen og en api key som skal brukes for å autentisere konsumenten mot rest-gateway. 

Navnformat for gateway url og api key:
- Gateway url: `<applikasjonsnavn>Api`.
- Api key: `<konsumentnavn>-<applikasjonsnavn>-apiKey`.
