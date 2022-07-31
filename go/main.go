package main

import (
	"database/sql"

	db2 "github.com/praxiscode/go-hexagonal/adapters/db"
	"github.com/praxiscode/go-hexagonal/application"
)

func main() {
	db, _ := sql.Open("sqlite3", "sqlite.db")
	productDbAdapter := db2.NewProductDb(db)
	productService := application.NewProductService(productDbAdapter)

	product, _ := productService.Create("Product Exemplo", 30)
	productService.Enable(product)

}
