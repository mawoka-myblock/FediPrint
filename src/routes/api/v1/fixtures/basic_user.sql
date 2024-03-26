-- Creating both basic user and profile

INSERT INTO profile (id, username, server, server_id, display_name, summary, inbox, outbox, public_key,
                            registered_at, updated_at)
VALUES ('018e7b20-51bd-703a-96c6-9c70cc723c67', 'testuser', 'localhost.local',
        'http://localhost:3000/api/v1/user/testuser', 'testuser', '',
        'http://localhost:3000/api/v1/user/testuser/inbox', 'http://localhost:3000/api/v1/user/testuser/outbox', e'-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3MyEeSoghnygXkxoZErc
UfcMkWmIN17ah9qjuxfoG/m+etmofE1/2wdW0ykxhCCIKxPfFhUMwfdJHX8t5mCL
jgfOo/KWUF7clpB+s1g5jcq11Vx1RutovoTBu/2ZNJCO0khyyt5fFLCPmKe5CqyP
M4HHEJ2eKUXfa9ZwEGz8SpJlMXv4qYC0DgKGTUS6tAVYJJL2E4lsIrICHsp+Cu/t
bVGlpPm+MeqkOP8hEebhWTYT8VzF8btLInaodHVLgQNfkbzxxHmPQOlnufEo8AAb
96yNeeYbA2cSbqhamrTmok2/R7l5X185n1yX5JUwwhCEWQzXm8QtF0DHYxsiitF2
gwIDAQAB
-----END PUBLIC KEY-----
', '2024-03-26 14:18:19.452533 +00:00', '2024-03-26 14:18:19.452533 +00:00');


INSERT INTO account (id, registered_at, updated_at, password, email, verified, profile_id, private_key)
VALUES ('018e7b20-51e5-79c2-878e-02d01f941165', '2024-03-26 14:18:19.492916 +00:00',
        '2024-03-26 14:18:19.492916 +00:00',
        '$argon2id$v=19$m=19456,t=2,p=1$TawkGAvCCDcSiaMPQTS5kw$pudkL7Wu3M5V4uyGKMiLfU9kMj4S6zgkju6aXxKtc5E',
        'test@mawoka.eu', '018e7b20-51e5-7fd9-b45c-b770542c1976', '018e7b20-51bd-703a-96c6-9c70cc723c67', e'-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA3MyEeSoghnygXkxoZErcUfcMkWmIN17ah9qjuxfoG/m+etmo
fE1/2wdW0ykxhCCIKxPfFhUMwfdJHX8t5mCLjgfOo/KWUF7clpB+s1g5jcq11Vx1
RutovoTBu/2ZNJCO0khyyt5fFLCPmKe5CqyPM4HHEJ2eKUXfa9ZwEGz8SpJlMXv4
qYC0DgKGTUS6tAVYJJL2E4lsIrICHsp+Cu/tbVGlpPm+MeqkOP8hEebhWTYT8VzF
8btLInaodHVLgQNfkbzxxHmPQOlnufEo8AAb96yNeeYbA2cSbqhamrTmok2/R7l5
X185n1yX5JUwwhCEWQzXm8QtF0DHYxsiitF2gwIDAQABAoIBABRV8c2HB7/bMpn8
x5CVJH2YF1w9MAKJhe8FQBc3OHV7JwQBj/cC3Ee8AU8peBoHNQNirSVHgwlKUT2a
PQv+0FugdHO9IAL++aoUXNb/xS0+AklIXrTNcbQ6Xe2GidnBhqXvbkMo//DT4iXZ
LL3C7t17km4BVlPOz6kK8v9QSlxDu6i4f/vtCg2o+jpg72Ieb7CeHkZ+Du995ugo
+Z1ZSi0R/w7oDmUigNJ2bKMIWUaqeMmR+eTEMDKXLv54VhOEXCeH2gnbng99ghvL
KRrDuGYZPR/e7+MaC+ALLIaOoB5tBsR/P5BgZaq68LgEQREuJUJuc/X8Ob1xhIBH
RhvTOAECgYEA8nNtmkHd+hr1HisDVftUfLiaJktUv+U47gAh6+NYnOvTITzNe4Ct
2mo8vc1ME7ko3NHqtzXeKW0zLaWQ9PYY47dHSUl0WyoHDiQW/rpfbnYJe9EPIc0B
0IztI0ZpWurxZP+ZS+deA2CbgHGrhl2FJ93KybDlbbheQSQ7lEBVuwECgYEA6SNU
Z2DOaWviJq07uGX+pjEyXpm1NL3cj4lfYITGU8x4vmN11TfmhhOrLx+/xghldLQc
PO6AH43uayFeobQJiyBayX1TPtCq+gSUO0MqIPXG5dfUcGOcGOhjUF0nIJ+PtKPR
94lPgMIJYqXEIJO9eUwbnL4+nFoqiutqZZ0LxYMCgYEAtI8gFdxt2wBOgfwYjOUS
w5ubOV+qqwqjviOdH5Z9fMfnwX4NradkUiACQnvs51dixikS+wSEAigQEDOYY8YP
PS9WSI8Kt4XyISbhdS0JOzNu1qYX9d0+N7lsNXQPrCUCR2xPFwckrbE5vYtp6TQm
oUz84/34ryC6GRtJv5u1/QECgYBPdeQxq/QOSF/3MLoXGmRVvpTdW2RAiqmfHoIs
gnSaYSmSMpIZzwi6EAAbeqXBWGFECpNJTTzMBHMLfn8jfBp4mdl3rUhvM23i8yaW
oEi+nSZidlKIz9qHPsWCwY0xeFDhj6hjxyAa0YejYL5dIB3HNuJ+ZPUwEydG3AAS
jBlUAwKBgQCWd9RZET8GduWXGhbTjasJhL8egYD8nRaWHPKsSUwt3EMPodnl5xrY
9oiBsErXabhiwNqa6+kbWfPgsTjQ17B4tdWRCk7vjaBjxG2f1MR4u2Omj47nHJ3f
JItFyrOGxgMbSRh0LoqCrkjt4259G7YBPfsfk2ZfH1xTWrUP7658oA==
-----END RSA PRIVATE KEY-----
');
