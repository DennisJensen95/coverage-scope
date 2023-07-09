FROM dennisjensen95/coverage-scope:v0.3.3

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
