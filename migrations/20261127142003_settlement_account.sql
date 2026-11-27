-- Add migration script here
INSERT INTO accounts (name, currency, allow_negative)
    VALUES
        ('Settlement (GBP)', 'GBP', true),
        ('Settlement (USD)', 'USD', true),
        ('Settlement (EUR)', 'EUR', true),
        ('Settlement (NGN)', 'NGN', true),
        ('Settlement (CAD)', 'CAD', true);