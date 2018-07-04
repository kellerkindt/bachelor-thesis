﻿$(function () {
    var mapData = Highcharts.geojson(Highcharts.maps["custom/world"]);

    var respondents = $("#highmap-respondents");
    var allVisits = $("#highmap-all-visits");
    var providence = $("#highmap-providence");

    var respondentsData = [
        {
            "code": "US",
            "z": 20.6478
        },
        {
            "code": "IN",
            "z": 13.9499
        },
        {
            "code": "DE",
            "z": 6.5668
        },
        {
            "code": "GB",
            "z": 6.3248
        },
        {
            "code": "CA",
            "z": 3.4496
        },
        {
            "code": "RU",
            "z": 2.9169
        },
        {
            "code": "FR",
            "z": 2.6149
        },
        {
            "code": "BR",
            "z": 2.5468
        },
        {
            "code": "PL",
            "z": 2.1574
        },
        {
            "code": "AU",
            "z": 2.0517
        },
        {
            "code": "NL",
            "z": 1.8717
        },
        {
            "code": "ES",
            "z": 1.7985
        },
        {
            "code": "IT",
            "z": 1.5606
        },
        {
            "code": "UA",
            "z": 1.3003
        },
        {
            "code": "SE",
            "z": 1.1834
        },
        {
            "code": "PK",
            "z": 1.0675
        },
        {
            "code": "CN",
            "z": 1.0543
        },
        {
            "code": "CH",
            "z": 1.0269
        },
        {
            "code": "TR",
            "z": 1.0208
        },
        {
            "code": "IL",
            "z": 1.0197
        },
        {
            "code": "IR",
            "z": 0.9364
        },
        {
            "code": "RO",
            "z": 0.8062
        },
        {
            "code": "AT",
            "z": 0.8011
        },
        {
            "code": "CZ",
            "z": 0.7971
        },
        {
            "code": "BE",
            "z": 0.7554
        },
        {
            "code": "MX",
            "z": 0.7483
        },
        {
            "code": "BD",
            "z": 0.7086
        },
        {
            "code": "DK",
            "z": 0.6639
        },
        {
            "code": "ZA",
            "z": 0.6476
        },
        {
            "code": "ID",
            "z": 0.6405
        },
        {
            "code": "AR",
            "z": 0.6212
        },
        {
            "code": "NO",
            "z": 0.5744
        },
        {
            "code": "NZ",
            "z": 0.5663
        },
        {
            "code": "IE",
            "z": 0.5632
        },
        {
            "code": "PT",
            "z": 0.5368
        },
        {
            "code": "FI",
            "z": 0.5297
        },
        {
            "code": "PH",
            "z": 0.5287
        },
        {
            "code": "GR",
            "z": 0.5246
        },
        {
            "code": "HU",
            "z": 0.4778
        },
        {
            "code": "LK",
            "z": 0.4616
        },
        {
            "code": "BG",
            "z": 0.4321
        },
        {
            "code": "EG",
            "z": 0.426
        },
        {
            "code": "NG",
            "z": 0.4057
        },
        {
            "code": "SG",
            "z": 0.3823
        },
        {
            "code": "MY",
            "z": 0.3691
        },
        {
            "code": "JP",
            "z": 0.367
        },
        {
            "code": "RS",
            "z": 0.364
        },
        {
            "code": "BY",
            "z": 0.3447
        },
        {
            "code": "CO",
            "z": 0.3447
        },
        {
            "code": "VN",
            "z": 0.3365
        },
        {
            "code": "NP",
            "z": 0.2999
        },
        {
            "code": "LT",
            "z": 0.2613
        },
        {
            "code": "HR",
            "z": 0.245
        },
        {
            "code": "CL",
            "z": 0.242
        },
        {
            "code": "SK",
            "z": 0.242
        },
        {
            "code": "SI",
            "z": 0.242
        },
        {
            "code": "HK",
            "z": 0.2227
        },
        {
            "code": "MA",
            "z": 0.2166
        },
        {
            "code": "TH",
            "z": 0.2166
        },
        {
            "code": "TW",
            "z": 0.2105
        },
        {
            "code": "KE",
            "z": 0.1972
        },
        {
            "code": "AE",
            "z": 0.1962
        },
        {
            "code": "EE",
            "z": 0.1922
        },
        {
            "code": "KR",
            "z": 0.1718
        },
        {
            "code": "TN",
            "z": 0.1657
        },
        {
            "code": "LV",
            "z": 0.1474
        },
        {
            "code": "DZ",
            "z": 0.1322
        },
        {
            "code": "SA",
            "z": 0.1322
        },
        {
            "code": "PE",
            "z": 0.1301
        },
        {
            "code": "BA",
            "z": 0.1271
        },
        {
            "code": "VE",
            "z": 0.1251
        },
        {
            "code": "AM",
            "z": 0.119
        },
        {
            "code": "DO",
            "z": 0.1169
        },
        {
            "code": "AL",
            "z": 0.1108
        },
        {
            "code": "KZ",
            "z": 0.1088
        },
        {
            "code": "LB",
            "z": 0.1088
        },
        {
            "code": "UY",
            "z": 0.1037
        },
        {
            "code": "CR",
            "z": 0.0996
        },
        {
            "code": "JO",
            "z": 0.0844
        },
        {
            "code": "AZ",
            "z": 0.0793
        },
        {
            "code": "GH",
            "z": 0.0773
        },
        {
            "code": "MD",
            "z": 0.0773
        },
        {
            "code": "GE",
            "z": 0.0763
        },
        {
            "code": "UG",
            "z": 0.0681
        },
        {
            "code": "MT",
            "z": 0.0671
        },
        {
            "code": "CU",
            "z": 0.0661
        },
        {
            "code": "EC",
            "z": 0.0661
        },
        {
            "code": "AF",
            "z": 0.0651
        },
        {
            "code": "KR",
            "z": 0.063
        },
        {
            "code": "KH",
            "z": 0.061
        },
        {
            "code": "ET",
            "z": 0.061
        },
        {
            "code": "LU",
            "z": 0.06
        },
        {
            "code": "SY",
            "z": 0.0569
        },
        {
            "code": "UZ",
            "z": 0.0569
        },
        {
            "code": "MM",
            "z": 0.0559
        },
        {
            "code": "MK",
            "z": 0.0549
        },
        {
            "code": "GT",
            "z": 0.0508
        },
        {
            "code": "CY",
            "z": 0.0498
        },
        {
            "code": "PY",
            "z": 0.0498
        },
        {
            "code": "TZ",
            "z": 0.0498
        },
        {
            "code": "SV",
            "z": 0.0468
        },
        {
            "code": "IS",
            "z": 0.0458
        },
        {
            "code": "BO",
            "z": 0.0447
        },
        {
            "code": "IQ",
            "z": 0.0427
        },
        {
            "code": "ZW",
            "z": 0.0397
        },
        {
            "code": "MU",
            "z": 0.0356
        },
        {
            "code": "CM",
            "z": 0.0346
        },
        {
            "code": "PA",
            "z": 0.0325
        },
        {
            "code": "NI",
            "z": 0.0315
        },
        {
            "code": "HN",
            "z": 0.0254
        },
        {
            "code": "SD",
            "z": 0.0254
        },
        {
            "code": "KW",
            "z": 0.0234
        },
        {
            "code": "QA",
            "z": 0.0234
        },
        {
            "code": "KG",
            "z": 0.0224
        },
        {
            "code": "MN",
            "z": 0.0214
        },
        {
            "code": "MZ",
            "z": 0.0214
        },
        {
            "code": "JM",
            "z": 0.0203
        },
        {
            "code": "OM",
            "z": 0.0203
        },
        {
            "code": "TT",
            "z": 0.0203
        },
        {
            "code": "MG",
            "z": 0.0193
        },
        {
            "code": "RW",
            "z": 0.0183
        },
        {
            "code": "BH",
            "z": 0.0163
        },
        {
            "code": "AD",
            "z": 0.0153
        },
        {
            "code": "ME",
            "z": 0.0153
        },
        {
            "code": "LY",
            "z": 0.0142
        },
        {
            "code": "SN",
            "z": 0.0132
        },
        {
            "code": "YE",
            "z": 0.0132
        },
        {
            "code": "AO",
            "z": 0.0112
        },
        {
            "code": "BJ",
            "z": 0.0102
        },
        {
            "code": "MV",
            "z": 0.0102
        },
        {
            "code": "SO",
            "z": 0.0102
        },
        {
            "code": "AG",
            "z": 0.0092
        },
        {
            "code": "MW",
            "z": 0.0092
        },
        {
            "code": "ZM",
            "z": 0.0092
        },
        {
            "code": "FJ",
            "z": 0.0081
        },
        {
            "code": "HT",
            "z": 0.0081
        },
        {
            "code": "CI",
            "z": 0.0071
        },
        {
            "code": "CD",
            "z": 0.0071
        },
        {
            "code": "BS",
            "z": 0.0061
        },
        {
            "code": "BT",
            "z": 0.0061
        },
        {
            "code": "CG",
            "z": 0.0061
        },
        {
            "code": "NA",
            "z": 0.0061
        },
        {
            "code": "TM",
            "z": 0.0061
        },
        {
            "code": "BF",
            "z": 0.0051
        },
        {
            "code": "GA",
            "z": 0.0051
        },
        {
            "code": "GM",
            "z": 0.0051
        },
        {
            "code": "LI",
            "z": 0.0051
        },
        {
            "code": "TG",
            "z": 0.0051
        },
        {
            "code": "BB",
            "z": 0.0041
        },
        {
            "code": "BW",
            "z": 0.0041
        },
        {
            "code": "MC",
            "z": 0.0041
        },
        {
            "code": "KP",
            "z": 0.0041
        },
        {
            "code": "TJ",
            "z": 0.0041
        },
        {
            "code": "CV",
            "z": 0.0031
        },
        {
            "code": "GN",
            "z": 0.0031
        },
        {
            "code": "LR",
            "z": 0.0031
        },
        {
            "code": "MR",
            "z": 0.0031
        },
        {
            "code": "SZ",
            "z": 0.0031
        },
        {
            "code": "CF",
            "z": 0.002
        },
        {
            "code": "KP",
            "z": 0.002
        },
        {
            "code": "DM",
            "z": 0.002
        },
        {
            "code": "GY",
            "z": 0.002
        },
        {
            "code": "LS",
            "z": 0.002
        },
        {
            "code": "MH",
            "z": 0.002
        },
        {
            "code": "FM",
            "z": 0.002
        },
        {
            "code": "NE",
            "z": 0.002
        },
        {
            "code": "PW",
            "z": 0.002
        },
        {
            "code": "SR",
            "z": 0.002
        },
        {
            "code": "BZ",
            "z": 0.001
        },
        {
            "code": "BN",
            "z": 0.001
        },
        {
            "code": "BI",
            "z": 0.001
        },
        {
            "code": "DJ",
            "z": 0.001
        },
        {
            "code": "ER",
            "z": 0.001
        },
        {
            "code": "GD",
            "z": 0.001
        },
        {
            "code": "GW",
            "z": 0.001
        },
        {
            "code": "ML",
            "z": 0.001
        },
        {
            "code": "NR",
            "z": 0.001
        },
        {
            "code": "LC",
            "z": 0.001
        },
        {
            "code": "SM",
            "z": 0.001
        },
        {
            "code": "SL",
            "z": 0.001
        },
        {
            "code": "SB",
            "z": 0.001
        },
        {
            "code": "TL",
            "z": 0.001
        }
    ];

    var allVisitsData = [
        {
            "code": "US",
            "z": 23.6368
        },
        {
            "code": "IN",
            "z": 13.5597
        },
        {
            "code": "GB",
            "z": 5.1364
        },
        {
            "code": "DE",
            "z": 4.9508
        },
        {
            "code": "CA",
            "z": 3.5329
        },
        {
            "code": "FR",
            "z": 3.1189
        },
        {
            "code": "RU",
            "z": 2.3958
        },
        {
            "code": "BR",
            "z": 2.3088
        },
        {
            "code": "PL",
            "z": 2.0428
        },
        {
            "code": "IT",
            "z": 1.9467
        },
        {
            "code": "ES",
            "z": 1.944
        },
        {
            "code": "NL",
            "z": 1.7367
        },
        {
            "code": "AU",
            "z": 1.5833
        },
        {
            "code": "VN",
            "z": 1.3486
        },
        {
            "code": "UA",
            "z": 1.2789
        },
        {
            "code": "CN",
            "z": 1.2349
        },
        {
            "code": "KR",
            "z": 1.2069
        },
        {
            "code": "ID",
            "z": 1.1288
        },
        {
            "code": "SE",
            "z": 1.1091
        },
        {
            "code": "JP",
            "z": 1.0781
        },
        {
            "code": "TR",
            "z": 0.9891
        },
        {
            "code": "TW",
            "z": 0.9697
        },
        {
            "code": "SG",
            "z": 0.9388
        },
        {
            "code": "MX",
            "z": 0.9053
        },
        {
            "code": "IL",
            "z": 0.8714
        },
        {
            "code": "PK",
            "z": 0.8166
        },
        {
            "code": "RO",
            "z": 0.8045
        },
        {
            "code": "CH",
            "z": 0.8029
        },
        {
            "code": "HK",
            "z": 0.7188
        },
        {
            "code": "PH",
            "z": 0.6912
        },
        {
            "code": "TH",
            "z": 0.6489
        },
        {
            "code": "CZ",
            "z": 0.6272
        },
        {
            "code": "BE",
            "z": 0.616
        },
        {
            "code": "AT",
            "z": 0.6106
        },
        {
            "code": "MY",
            "z": 0.6052
        },
        {
            "code": "PT",
            "z": 0.5806
        },
        {
            "code": "DK",
            "z": 0.5562
        },
        {
            "code": "FI",
            "z": 0.5398
        },
        {
            "code": "AR",
            "z": 0.5259
        },
        {
            "code": "HU",
            "z": 0.486
        },
        {
            "code": "ZA",
            "z": 0.4814
        },
        {
            "code": "NO",
            "z": 0.4653
        },
        {
            "code": "IE",
            "z": 0.4027
        },
        {
            "code": "BY",
            "z": 0.3912
        },
        {
            "code": "GR",
            "z": 0.3874
        },
        {
            "code": "BG",
            "z": 0.3865
        },
        {
            "code": "EG",
            "z": 0.3505
        },
        {
            "code": "NZ",
            "z": 0.3439
        },
        {
            "code": "RS",
            "z": 0.3335
        },
        {
            "code": "CL",
            "z": 0.3266
        },
        {
            "code": "CO",
            "z": 0.2883
        },
        {
            "code": "BD",
            "z": 0.2759
        },
        {
            "code": "SK",
            "z": 0.2422
        },
        {
            "code": "AE",
            "z": 0.2277
        },
        {
            "code": "HR",
            "z": 0.2102
        },
        {
            "code": "LT",
            "z": 0.2075
        },
        {
            "code": "LK",
            "z": 0.2059
        },
        {
            "code": "MA",
            "z": 0.1943
        },
        {
            "code": "PE",
            "z": 0.1931
        },
        {
            "code": "SA",
            "z": 0.1853
        },
        {
            "code": "SI",
            "z": 0.1601
        },
        {
            "code": "EE",
            "z": 0.154
        },
        {
            "code": "LV",
            "z": 0.1396
        },
        {
            "code": "NP",
            "z": 0.1191
        },
        {
            "code": "NG",
            "z": 0.1082
        },
        {
            "code": "EC",
            "z": 0.1077
        },
        {
            "code": "KZ",
            "z": 0.1064
        },
        {
            "code": "BA",
            "z": 0.0967
        },
        {
            "code": "AM",
            "z": 0.0965
        },
        {
            "code": "LB",
            "z": 0.0962
        },
        {
            "code": "CR",
            "z": 0.0961
        },
        {
            "code": "JO",
            "z": 0.0874
        },
        {
            "code": "VE",
            "z": 0.0869
        },
        {
            "code": "MK",
            "z": 0.0838
        },
        {
            "code": "UY",
            "z": 0.0808
        },
        {
            "code": "MD",
            "z": 0.0765
        },
        {
            "code": "KE",
            "z": 0.0742
        },
        {
            "code": "DO",
            "z": 0.0587
        },
        {
            "code": "KH",
            "z": 0.0585
        },
        {
            "code": "IR",
            "z": 0.0585
        },
        {
            "code": "GE",
            "z": 0.058
        },
        {
            "code": "MM",
            "z": 0.0567
        },
        {
            "code": "MT",
            "z": 0.0565
        },
        {
            "code": "BO",
            "z": 0.0517
        },
        {
            "code": "IS",
            "z": 0.0488
        },
        {
            "code": "AZ",
            "z": 0.0453
        },
        {
            "code": "PS",
            "z": 0.0438
        },
        {
            "code": "GT",
            "z": 0.0427
        },
        {
            "code": "LU",
            "z": 0.0419
        },
        {
            "code": "CY",
            "z": 0.041
        },
        {
            "code": "TN",
            "z": 0.0393
        },
        {
            "code": "AL",
            "z": 0.0386
        },
        {
            "code": "KW",
            "z": 0.0371
        },
        {
            "code": "QA",
            "z": 0.037
        },
        {
            "code": "SV",
            "z": 0.0336
        },
        {
            "code": "MN",
            "z": 0.0295
        },
        {
            "code": "DZ",
            "z": 0.0295
        },
        {
            "code": "IQ",
            "z": 0.0285
        },
        {
            "code": "UZ",
            "z": 0.0265
        },
        {
            "code": "PY",
            "z": 0.0264
        },
        {
            "code": "GH",
            "z": 0.0261
        },
        {
            "code": "MU",
            "z": 0.0256
        },
        {
            "code": "PR",
            "z": 0.0255
        },
        {
            "code": "ET",
            "z": 0.0216
        },
        {
            "code": "CU",
            "z": 0.0215
        },
        {
            "code": "MO",
            "z": 0.0212
        },
        {
            "code": "KG",
            "z": 0.021
        },
        {
            "code": "BH",
            "z": 0.0199
        },
        {
            "code": "JM",
            "z": 0.0195
        },
        {
            "code": "NI",
            "z": 0.0192
        },
        {
            "code": "UG",
            "z": 0.0188
        },
        {
            "code": "PA",
            "z": 0.0182
        },
        {
            "code": "HN",
            "z": 0.0176
        },
        {
            "code": "OM",
            "z": 0.016
        },
        {
            "code": "TZ",
            "z": 0.0144
        },
        {
            "code": "TT",
            "z": 0.0125
        },
        {
            "code": "MG",
            "z": 0.0105
        },
        {
            "code": "ME",
            "z": 0.0105
        },
        {
            "code": "SN",
            "z": 0.0098
        },
        {
            "code": "MV",
            "z": 0.0088
        },
        {
            "code": "CI",
            "z": 0.0088
        },
        {
            "code": "CM",
            "z": 0.0083
        },
        {
            "code": "RW",
            "z": 0.0079
        },
        {
            "code": "BN",
            "z": 0.0076
        },
        {
            "code": "ZW",
            "z": 0.0067
        },
        {
            "code": "RE",
            "z": 0.0066
        },
        {
            "code": "LY",
            "z": 0.0066
        },
        {
            "code": "LA",
            "z": 0.006
        },
        {
            "code": "XK",
            "z": 0.0058
        },
        {
            "code": "JE",
            "z": 0.0057
        },
        {
            "code": "MZ",
            "z": 0.0054
        },
        {
            "code": "AO",
            "z": 0.0046
        },
        {
            "code": "NA",
            "z": 0.0044
        },
        {
            "code": "BW",
            "z": 0.0043
        },
        {
            "code": "BB",
            "z": 0.0043
        },
        {
            "code": "AF",
            "z": 0.0042
        },
        {
            "code": "ZM",
            "z": 0.0038
        },
        {
            "code": "IM",
            "z": 0.0037
        },
        {
            "code": "AD",
            "z": 0.0036
        },
        {
            "code": "NC",
            "z": 0.0035
        },
        {
            "code": "SR",
            "z": 0.0034
        },
        {
            "code": "YE",
            "z": 0.0033
        },
        {
            "code": "CW",
            "z": 0.0032
        },
        {
            "code": "AX",
            "z": 0.0031
        },
        {
            "code": "HT",
            "z": 0.0031
        },
        {
            "code": "LI",
            "z": 0.0031
        },
        {
            "code": "GP",
            "z": 0.003
        },
        {
            "code": "GI",
            "z": 0.0028
        },
        {
            "code": "GG",
            "z": 0.0028
        },
        {
            "code": "PF",
            "z": 0.0027
        },
        {
            "code": "BJ",
            "z": 0.0027
        },
        {
            "code": "FO",
            "z": 0.0026
        },
        {
            "code": "KY",
            "z": 0.0025
        },
        {
            "code": "BT",
            "z": 0.0025
        },
        {
            "code": "BS",
            "z": 0.0025
        },
        {
            "code": "MQ",
            "z": 0.0023
        },
        {
            "code": "FJ",
            "z": 0.0022
        },
        {
            "code": "TG",
            "z": 0.0021
        },
        {
            "code": "BM",
            "z": 0.0021
        },
        {
            "code": "CD",
            "z": 0.002
        },
        {
            "code": "TJ",
            "z": 0.002
        },
        {
            "code": "BF",
            "z": 0.0019
        },
        {
            "code": "BZ",
            "z": 0.0018
        },
        {
            "code": "GY",
            "z": 0.0018
        },
        {
            "code": "SO",
            "z": 0.0014
        },
        {
            "code": "GA",
            "z": 0.0014
        },
        {
            "code": "TM",
            "z": 0.0013
        },
        {
            "code": "LS",
            "z": 0.0013
        },
        {
            "code": "SM",
            "z": 0.0013
        },
        {
            "code": "ML",
            "z": 0.0013
        },
        {
            "code": "SD",
            "z": 0.0012
        },
        {
            "code": "SY",
            "z": 0.0012
        },
        {
            "code": "SZ",
            "z": 0.0012
        },
        {
            "code": "GU",
            "z": 0.0012
        },
        {
            "code": "AW",
            "z": 0.0011
        },
        {
            "code": "LC",
            "z": 0.0011
        },
        {
            "code": "PG",
            "z": 0.001
        },
        {
            "code": "MW",
            "z": 0.001
        },
        {
            "code": "MR",
            "z": 0.0009
        },
        {
            "code": "SC",
            "z": 0.0008
        },
        {
            "code": "AG",
            "z": 0.0008
        },
        {
            "code": "KN",
            "z": 0.0006
        },
        {
            "code": "GD",
            "z": 0.0005
        }
    ];

    var providenceData = [
        {
            "code": "US",
            "z": 23.5047
        },
        {
            "code": "IN",
            "z": 14.15
        },
        {
            "code": "DE",
            "z": 5.455
        },
        {
            "code": "GB",
            "z": 5.1555
        },
        {
            "code": "CA",
            "z": 3.1355
        },
        {
            "code": "CN",
            "z": 3.1262
        },
        {
            "code": "FR",
            "z": 3.1115
        },
        {
            "code": "BR",
            "z": 2.2632
        },
        {
            "code": "IT",
            "z": 2.2061
        },
        {
            "code": "RU",
            "z": 2.0848
        },
        {
            "code": "ES",
            "z": 1.9109
        },
        {
            "code": "NL",
            "z": 1.7637
        },
        {
            "code": "AU",
            "z": 1.7436
        },
        {
            "code": "PL",
            "z": 1.7065
        },
        {
            "code": "JP",
            "z": 1.3897
        },
        {
            "code": "ID",
            "z": 1.2802
        },
        {
            "code": "MX",
            "z": 1.0638
        },
        {
            "code": "PH",
            "z": 1.0367
        },
        {
            "code": "KR",
            "z": 0.9998
        },
        {
            "code": "UA",
            "z": 0.9755
        },
        {
            "code": "SE",
            "z": 0.9506
        },
        {
            "code": "VN",
            "z": 0.9408
        },
        {
            "code": "TR",
            "z": 0.8053
        },
        {
            "code": "PK",
            "z": 0.7975
        },
        {
            "code": "TW",
            "z": 0.7813
        },
        {
            "code": "CH",
            "z": 0.7519
        },
        {
            "code": "SG",
            "z": 0.7166
        },
        {
            "code": "IL",
            "z": 0.704
        },
        {
            "code": "MY",
            "z": 0.6815
        },
        {
            "code": "TH",
            "z": 0.663
        },
        {
            "code": "RO",
            "z": 0.6428
        },
        {
            "code": "BE",
            "z": 0.6395
        },
        {
            "code": "HK",
            "z": 0.6013
        },
        {
            "code": "AT",
            "z": 0.5826
        },
        {
            "code": "AR",
            "z": 0.5456
        },
        {
            "code": "CZ",
            "z": 0.5302
        },
        {
            "code": "PT",
            "z": 0.5106
        },
        {
            "code": "DK",
            "z": 0.5032
        },
        {
            "code": "ZA",
            "z": 0.4929
        },
        {
            "code": "FI",
            "z": 0.4634
        },
        {
            "code": "HU",
            "z": 0.4533
        },
        {
            "code": "EG",
            "z": 0.3971
        },
        {
            "code": "NO",
            "z": 0.3971
        },
        {
            "code": "IE",
            "z": 0.3444
        },
        {
            "code": "GR",
            "z": 0.3126
        },
        {
            "code": "CO",
            "z": 0.3078
        },
        {
            "code": "BG",
            "z": 0.2926
        },
        {
            "code": "CL",
            "z": 0.2908
        },
        {
            "code": "NZ",
            "z": 0.2899
        },
        {
            "code": "RS",
            "z": 0.2823
        },
        {
            "code": "BY",
            "z": 0.2746
        },
        {
            "code": "BD",
            "z": 0.2431
        },
        {
            "code": "MA",
            "z": 0.2389
        },
        {
            "code": "SK",
            "z": 0.2162
        },
        {
            "code": "LK",
            "z": 0.2125
        },
        {
            "code": "AE",
            "z": 0.2021
        },
        {
            "code": "SA",
            "z": 0.1966
        },
        {
            "code": "HR",
            "z": 0.1896
        },
        {
            "code": "PE",
            "z": 0.1816
        },
        {
            "code": "NG",
            "z": 0.1485
        },
        {
            "code": "LT",
            "z": 0.1451
        },
        {
            "code": "SI",
            "z": 0.123
        },
        {
            "code": "EC",
            "z": 0.1141
        },
        {
            "code": "VE",
            "z": 0.111
        },
        {
            "code": "EE",
            "z": 0.1094
        },
        {
            "code": "CR",
            "z": 0.1081
        },
        {
            "code": "LV",
            "z": 0.0991
        },
        {
            "code": "NP",
            "z": 0.0954
        },
        {
            "code": "KZ",
            "z": 0.0929
        },
        {
            "code": "IR",
            "z": 0.0861
        },
        {
            "code": "KE",
            "z": 0.0795
        },
        {
            "code": "BA",
            "z": 0.0736
        },
        {
            "code": "AM",
            "z": 0.0686
        },
        {
            "code": "LB",
            "z": 0.0686
        },
        {
            "code": "UY",
            "z": 0.0672
        },
        {
            "code": "MK",
            "z": 0.0647
        },
        {
            "code": "JO",
            "z": 0.0636
        },
        {
            "code": "MD",
            "z": 0.0573
        },
        {
            "code": "MT",
            "z": 0.0568
        },
        {
            "code": "DO",
            "z": 0.0555
        },
        {
            "code": "DZ",
            "z": 0.0536
        },
        {
            "code": "BO",
            "z": 0.0523
        },
        {
            "code": "MM",
            "z": 0.0503
        },
        {
            "code": "KH",
            "z": 0.0502
        },
        {
            "code": "TN",
            "z": 0.0483
        },
        {
            "code": "GT",
            "z": 0.0454
        },
        {
            "code": "LU",
            "z": 0.0409
        },
        {
            "code": "PS",
            "z": 0.0396
        },
        {
            "code": "MU",
            "z": 0.0387
        },
        {
            "code": "SV",
            "z": 0.0383
        },
        {
            "code": "GE",
            "z": 0.0377
        },
        {
            "code": "IS",
            "z": 0.0369
        },
        {
            "code": "AL",
            "z": 0.0356
        },
        {
            "code": "GH",
            "z": 0.0355
        },
        {
            "code": "PR",
            "z": 0.0351
        },
        {
            "code": "KW",
            "z": 0.0337
        },
        {
            "code": "AZ",
            "z": 0.0325
        },
        {
            "code": "QA",
            "z": 0.0323
        },
        {
            "code": "CY",
            "z": 0.0323
        },
        {
            "code": "IQ",
            "z": 0.0302
        },
        {
            "code": "ET",
            "z": 0.0302
        },
        {
            "code": "PY",
            "z": 0.0254
        },
        {
            "code": "TZ",
            "z": 0.0241
        },
        {
            "code": "UZ",
            "z": 0.0227
        },
        {
            "code": "BH",
            "z": 0.022
        },
        {
            "code": "MN",
            "z": 0.0219
        },
        {
            "code": "MO",
            "z": 0.0216
        },
        {
            "code": "JM",
            "z": 0.0215
        },
        {
            "code": "UG",
            "z": 0.021
        },
        {
            "code": "HN",
            "z": 0.02
        },
        {
            "code": "PA",
            "z": 0.0193
        },
        {
            "code": "CU",
            "z": 0.019
        },
        {
            "code": "OM",
            "z": 0.0179
        },
        {
            "code": "NI",
            "z": 0.0176
        },
        {
            "code": "KG",
            "z": 0.0144
        },
        {
            "code": "TT",
            "z": 0.0139
        },
        {
            "code": "MG",
            "z": 0.0135
        },
        {
            "code": "SN",
            "z": 0.0131
        },
        {
            "code": "CI",
            "z": 0.0127
        },
        {
            "code": "CM",
            "z": 0.011
        },
        {
            "code": "ME",
            "z": 0.0095
        },
        {
            "code": "RE",
            "z": 0.009
        },
        {
            "code": "ZW",
            "z": 0.0088
        },
        {
            "code": "LY",
            "z": 0.0087
        },
        {
            "code": "RW",
            "z": 0.0084
        },
        {
            "code": "YE",
            "z": 0.0079
        },
        {
            "code": "LA",
            "z": 0.0078
        },
        {
            "code": "MV",
            "z": 0.0077
        },
        {
            "code": "BN",
            "z": 0.0074
        },
        {
            "code": "ZM",
            "z": 0.0063
        },
        {
            "code": "XK",
            "z": 0.0061
        },
        {
            "code": "AO",
            "z": 0.006
        },
        {
            "code": "BB",
            "z": 0.0059
        },
        {
            "code": "MZ",
            "z": 0.0057
        },
        {
            "code": "GP",
            "z": 0.0053
        },
        {
            "code": "AF",
            "z": 0.0052
        },
        {
            "code": "BW",
            "z": 0.0051
        },
        {
            "code": "NA",
            "z": 0.0046
        },
        {
            "code": "JE",
            "z": 0.0046
        },
        {
            "code": "BJ",
            "z": 0.0043
        },
        {
            "code": "HT",
            "z": 0.004
        },
        {
            "code": "MQ",
            "z": 0.0038
        },
        {
            "code": "IM",
            "z": 0.0038
        },
        {
            "code": "CW",
            "z": 0.0036
        },
        {
            "code": "SR",
            "z": 0.0036
        },
        {
            "code": "AD",
            "z": 0.0035
        },
        {
            "code": "LI",
            "z": 0.0031
        },
        {
            "code": "SY",
            "z": 0.003
        },
        {
            "code": "PF",
            "z": 0.003
        },
        {
            "code": "TG",
            "z": 0.0029
        },
        {
            "code": "GI",
            "z": 0.0028
        },
        {
            "code": "BZ",
            "z": 0.0028
        },
        {
            "code": "NC",
            "z": 0.0027
        },
        {
            "code": "BT",
            "z": 0.0027
        },
        {
            "code": "BS",
            "z": 0.0026
        },
        {
            "code": "CD",
            "z": 0.0025
        },
        {
            "code": "GG",
            "z": 0.0025
        },
        {
            "code": "BF",
            "z": 0.0025
        },
        {
            "code": "TJ",
            "z": 0.0023
        },
        {
            "code": "MW",
            "z": 0.0023
        },
        {
            "code": "SD",
            "z": 0.0022
        },
        {
            "code": "AX",
            "z": 0.0022
        },
        {
            "code": "FJ",
            "z": 0.0021
        },
        {
            "code": "SO",
            "z": 0.002
        },
        {
            "code": "FO",
            "z": 0.002
        },
        {
            "code": "KY",
            "z": 0.002
        },
        {
            "code": "PG",
            "z": 0.0019
        },
        {
            "code": "LS",
            "z": 0.0019
        },
        {
            "code": "SZ",
            "z": 0.0019
        },
        {
            "code": "GY",
            "z": 0.0018
        },
        {
            "code": "BM",
            "z": 0.0018
        },
        {
            "code": "GA",
            "z": 0.0017
        },
        {
            "code": "GU",
            "z": 0.0016
        },
        {
            "code": "ML",
            "z": 0.0016
        },
        {
            "code": "AW",
            "z": 0.0014
        },
        {
            "code": "LC",
            "z": 0.0013
        },
        {
            "code": "SM",
            "z": 0.0012
        },
        {
            "code": "MR",
            "z": 0.0012
        },
        {
            "code": "TM",
            "z": 0.0011
        },
        {
            "code": "SC",
            "z": 0.001
        },
        {
            "code": "AG",
            "z": 0.001
        },
        {
            "code": "GM",
            "z": 0.0009
        },
        {
            "code": "GN",
            "z": 0.0008
        },
        {
            "code": "NE",
            "z": 0.0007
        },
        {
            "code": "GD",
            "z": 0.0006
        },
        {
            "code": "GF",
            "z": 0.0006
        },
        {
            "code": "KN",
            "z": 0.0005
        },
        {
            "code": "VC",
            "z": 0.0005
        }
    ];

    respondents.highcharts("Map", createMap("Respondents", respondentsData, "survey respondents"));
    allVisits.highcharts("Map", createMap("All Visits", allVisitsData, "Stack Overflow traffic"));
    providence.highcharts("Map", createMap("Professional Developers", providenceData, "professional developers"));

    function createMap(name, data, hoverSuffix) {
        return {
            chart: {
                borderWidth: 1,
                borderColor: "#F7F7F7"
            },

            credits: {
                enabled: false
            },

            title: {
                text: ""
            },

            legend: {
                enabled: false
            },

            mapNavigation: {
                enabled: true,
                buttonOptions: {
                    verticalAlign: "bottom"
                },
                enableMouseWheelZoom: false
            },

            series: [{
                    name: "Countries",
                    mapData: mapData,
                    color: "#E0E0E0",
                    enableMouseTracking: false
                },
                {
                    type: "mapbubble",
                    mapData: mapData,
                    color: "#54B7FF",
                    name: name,
                    data: data,
                    joinBy: ["iso-a2", "code"],
                    minSize: 4,
                    maxSize: "20%",
                    tooltip: {
                        pointFormatter: function () {
                            if (this.z === 1) {
                                return this.name + ": " + Highcharts.numberFormat(this.z, 4, ".", ",") + "% of " + hoverSuffix;
                            } else {
                                return this.name + ": " + Highcharts.numberFormat(this.z, 4, ".", ",") + "% of " + hoverSuffix;
                            }
                        }
                    }
                }
            ]
        };
    }
});