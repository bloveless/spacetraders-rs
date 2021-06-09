#[test]
fn parses_location_info() {
    let response = "
        {
            \"location\":{
                \"symbol\":\"OE-XV-91-2\",
                \"type\":\"WORMHOLE\",
                \"name\":\"Wormhole\",
                \"x\":0,
                \"y\":145,
                \"ships\":[

                ],
                \"messages\":[
                    \"Extensive research has revealed a partially functioning warp gate harnessing the power of an unstable but traversable wormhole.\",
                    \"The scientific community has determined a means of stabilizing the ancient structure.\",
                    \"Enter at your own risk.\",
                    \"POST https://api.spacetraders.io/game/structures/:structureId/deposit shipId=:shipId good=:goodSymbol quantity=:quantity\",
                    \"POST https://api.spacetraders.io/users/{username}/warp-jump shipId=:shipId\"
                ],
                \"structures\":[
                {
                    \"id\":\"ckmu843xh1451mlwwgkfvmpdw\",
                    \"name\":\"Warp Gate\",
                    \"completed\":true,
                    \"materials\":[
                    {
                        \"good\":\"ELECTRONICS\",
                        \"quantity\":9912,
                        \"targetQuantity\":20000
                    },
                    {
                        \"good\":\"MACHINERY\",
                        \"quantity\":15630,
                        \"targetQuantity\":55000
                    },
                    {
                        \"good\":\"CONSTRUCTION_MATERIALS\",
                        \"quantity\":25000,
                        \"targetQuantity\":25000
                    },
                    {
                        \"good\":\"METALS\",
                        \"quantity\":248880,
                        \"targetQuantity\":250000
                    }
                    ],
                    \"stability\":0.8554914285714286
                }
                ]
            },
            \"dockedShips\":5
        }
    ";

    println!("{}", response);

    serde_json::from_str::<spacetraders::responses::LocationInfo>(&response).unwrap();
}

#[test]
fn parse_flight_plan() {
    let response = "{\"flightPlan\":{\"id\":\"ckn173p2b6294291bs607c85zst\",\"shipId\":\"ckn14lf9a1805601bs68mcck18x\",\"createdAt\":\"2021-04-03T03:47:51.588Z\",\"arrivesAt\":\"2021-04-03T04:03:26.569Z\",\"destination\":\"OE-XV-91-2\",\"departure\":\"OE-PM-TR\",\"distance\":175,\"fuelConsumed\":45,\"fuelRemaining\":55,\"terminatedAt\":null,\"timeRemainingInSeconds\":934}}";

    serde_json::from_str::<spacetraders::responses::FlightPlan>(&response).unwrap();
}
