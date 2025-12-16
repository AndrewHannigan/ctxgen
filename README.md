# ctxgen

A CLI tool that generates `AGENTS.md` and `CLAUDE.md` files from a `.context` folder.

## Installation

Install with Homebrew:
```
brew install andrewhannigan/tap/ctxgen
```

Install via shell script:
```
curl -LsSf https://github.com/AndrewHannigan/ctxgen/releases/latest/download/ctxgen-installer.sh | sh
```

## Overview

`ctxgen` reads text files from a `.context` directory (typically at the root of a code repository) and compiles them into a flat markdown files suitable for AI agents. The tool also supports progressive disclosure of context via the `<ctxgen:fold>` tag.

## Usage

```bash
# Basic usage (uses .context in current directory)
ctxgen

# Specify a custom context directory
ctxgen --context-dir /path/to/.context

# Specify output directory
ctxgen --output-dir /path/to/output

# Full options
ctxgen -c .context -o .
```

## Context Folder Structure

The `.context` folder can contain text files organized in any folder structure. File format, file naming, and directory structure do not matter. Organize the `.context` folder however you see fit for your project. Here's an example:

```
.context/
├── guidelines.txt
├── architecture/
│   ├── overview.txt
│   └── patterns.txt
└── api/
    └── conventions.txt
```

## Output Format

When compiled to the AGENTS.md and CLAUDE.md, each file is wrapped in a `<file>` tag:

```xml
<file path="guidelines.txt">
Content of the file...
</file>

<file path="architecture/overview.txt">
Content of the file...
</file>
```

## Fold Tags

`ctxgen` supports progressive disclosure of context via the `<ctxgen:fold>` tag. Content inside of `<ctxgen:fold>` tags are automatically replaced by a placeholder in the compiled instructions files. This placeholder describes the location of the folded content. The Agent can directly read the folded content using standard filesystem tools.

One example where the fold tag could be useful is a table schema file. Suppose we have a file `.context/tables/orders.txt` with the following contents:

```xml
The `public.orders` table contains a row for each order placed at the company.

Schema:
<ctxgen:fold>
| Column           | Type                        | Constraints            | Description                                    |
|------------------|-----------------------------|------------------------|------------------------------------------------|
| id               | BIGINT                      | PRIMARY KEY            | Unique identifier for the order                |
| customer_id      | BIGINT                      | NOT NULL, FK           | Reference to customers.id                      |
| order_number     | VARCHAR(50)                 | UNIQUE, NOT NULL       | Human-readable order number (e.g., "ORD-1234") |
| status           | VARCHAR(20)                 | NOT NULL               | Order status: confirmed, shipped, delivered    |
| total_amount     | DECIMAL(10,2)               | NOT NULL               | Total order amount in USD                      |
| subtotal         | DECIMAL(10,2)               | NOT NULL               | Subtotal before tax and shipping               |
| tax_amount       | DECIMAL(10,2)               | NOT NULL               | Total tax amount                               |
| shipping_amount  | DECIMAL(10,2)               | NOT NULL               | Shipping cost                                  |
| discount_amount  | DECIMAL(10,2)               | DEFAULT 0.00           | Total discount applied                         |
| currency         | CHAR(3)                     | DEFAULT 'USD'          | ISO 4217 currency code                         |
| payment_method   | VARCHAR(50)                 | NULL                   | Payment method used (credit_card, paypal, etc.)|
| shipping_address | JSONB                       | NOT NULL               | Shipping address details                       |
| billing_address  | JSONB                       | NOT NULL               | Billing address details                        |
| notes            | TEXT                        | NULL                   | Additional order notes                         |
| created_at       | TIMESTAMP WITH TIME ZONE    | NOT NULL, DEFAULT NOW()| Order creation timestamp                       |
| updated_at       | TIMESTAMP WITH TIME ZONE    | NOT NULL, DEFAULT NOW()| Last update timestamp                          |
| shipped_at       | TIMESTAMP WITH TIME ZONE    | NULL                   | Timestamp when order was shipped               |
| delivered_at     | TIMESTAMP WITH TIME ZONE    | NULL                   | Timestamp when order was delivered             |

Indexes:
- idx_orders_customer_id ON customer_id
- idx_orders_status ON status
- idx_orders_created_at ON created_at DESC
- idx_orders_order_number ON order_number (unique)

Foreign Keys:
- customer_id REFERENCES customers(id) ON DELETE RESTRICT
</ctxgen:fold>
```

When compiled, the content enclosed in the `<ctxgen:fold>` tags is replaced with a placeholder:

```xml
<file path="tables/orders.txt" has_folds="true">
The `public.orders` table contains a row for each order placed at the company.

Schema:
[Folded content: 30 lines (lines 5-34). Read file for full content.]
</file>
```

## License

MIT

