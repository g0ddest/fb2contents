# Парсер метаинформации fb2

Вытаскивает содержание из файла fb2, а так же метаинформацию. Возвращает все в формате json. Работает со всеми кодировками fb2.

Пример возвращаемого значения:

```json
{
  "description": {
    "title-info": {
      "genre": [
        "sf_fantasy"
      ],
      "author": [
        {
          "first-name": "Роберт",
          "last-name": "Сальваторе",
          "id": "74bff7b5-2a81-102a-9ae1-2dfe723fe7c7"
        }
      ],
      "book-title": "Заклятие короля-колдуна",
      "date": "2005",
      "coverpage": {
        "image": [
          {
            "l:href": "#cover.jpg"
          }
        ]
      },
      "lang": "ru",
      "src-lang": "en",
      "translator": [
        {
          "first-name": "Евгения",
          "last-name": "Фурсикова",
          "id": "a7a02e9d-2a82-102a-9ae1-2dfe723fe7c7"
        }
      ],
      "sequence": [
        {
          "name": "Наемные клинки",
          "number": "2"
        }
      ]
    },
    "document-info": {
      "author": [
        {
          "first-name": "",
          "last-name": "shum29",
          "nickname": "shum29"
        },
        {
          "first-name": "",
          "last-name": "MCat78",
          "nickname": "MCat78",
          "email": [
            "MCat78@mail.ru"
          ]
        }
      ],
      "program-used": "Book Designer 4.0, FB Writer v1.1",
      "date": "17.05.2007",
      "src-url": [
        "http://dragonrealms.narod.ru"
      ],
      "src-ocr": "Alex Mustaeff",
      "id": "3835ecdf-66df-102a-990a-1c76fd93e5c4",
      "version": "1.1",
      "history": {
        "$value": [
          "Скан и вычитка – Alex Mustaeff",
          "shum29 – создание – fb2",
          "v 1.1 – дополнительное форматирование, добавление информации – (MCat78)"
        ]
      }
    },
    "publish-info": {
      "book-name": "Заклятие короля-колдуна",
      "publisher": "Максима",
      "city": "Москва",
      "year": "2007",
      "isbn": "5-94955-100-1",
      "sequence": [
        {
          "name": "Забытые королевства",
          "number": "0"
        }
      ]
    },
    "custom-info": [
      {
        "info-type": "src-author-first-name",
        "$value": [
          "Robert A."
        ]
      },
      {
        "info-type": "src-author-last-name",
        "$value": [
          "Salvatore"
        ]
      },
      {
        "info-type": "src-book-title",
        "$value": [
          "Promise of the Witch-King"
        ]
      },
      {
        "info-type": "src-sequence-name",
        "$value": [
          "Forgotten Realms"
        ]
      }
    ]
  },
  "body": {
    "title": {
      "$value": [
        "Роберт Энтони Сальваторе",
        "Заклятие короля-колдуна"
      ]
    },
    "section": [
      {},
      {
        "title": {
          "$value": [
            "Пролог"
          ]
        }
      },
      {
        "title": {
          "$value": [
            "ЧАСТЬ ПЕРВАЯ",
            "Плоды интриг"
          ]
        },
        "section": [
          {},
          {
            "title": {
              "$value": [
                "Глава 1",
                "Охотники"
              ]
            }
          },
          {
            "title": {
              "$value": [
                "Глава 2",
                "Несносное отражение"
              ]
            }
          }
    ]
  }
}
```

Обратите внимание, что `description` не возвращается. Это намеренно сделано так, потому что люди там творят черт знает что.

Есть два режима работы — в формате cli, для этого надо надо запустить с ключами `cli %filename%`, к примеру `fb2contents cli book.fb2`.

Второй режим работы в виде веб-сервера, шлете POST на http://127.0.0.1:3000/parse с книгой в теле, а в ответ получаете или 200 с json, или 500 с json-ом с описанием ошибки. 