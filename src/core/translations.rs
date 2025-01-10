use literally::hmap;
use std::collections::HashMap;

use crate::Source;

pub fn return_mapping(source: Source) -> HashMap<String, String> {
    let product_mapping: HashMap<String, String> = hmap! {"Codigo"=> "codigo",
    "Descripcion"=> "descripcion",
    "presentacion"=> "presentacion",
    "Rubro"=> "rubro_id",
    "Observaciones"=> "observaciones",
    "Numero_Ingredientes"=> "numero_ingredientes",
    "Ingredient 1"=> "ingredient 1",
    "Ingredient 2"=> "ingredient 2",
    "Ingredient 3"=> "ingredient 3",
    "Ingredient 4"=> "ingredient 4",
    "Ingredient 5"=> "ingredient 5",
    "Ingredient 6"=> "ingredient 6",
    "Ingredient 7"=> "ingredient 7",
    "Ingredient 8"=> "ingredient 8",
    "Ingredient 9"=> "ingredient 9",
    "Ingredient 10"=> "ingredient 10",
    "Ingredient 11"=> "ingredient 11",
    "Ingredient 12"=> "ingredient 12",
    "Ingredient 13"=> "ingredient 13",
    "Ingredient 14"=> "ingredient 14",
    "Ingredient 15"=> "ingredient 15",
    "Ingredient 16"=> "ingredient 16",
    "Ingredient 17"=> "ingredient 17",
    "Ingredient 18"=> "ingredient 18",
    "Ingredient 19"=> "ingredient 19",
    "Ingredient 20"=> "ingredient 20",
    "Ingredient 21"=> "ingredient 21",
    "Ingredient 22"=> "ingredient 22",
    "Ingredient 23"=> "ingredient 23",
    "Ingredient 24"=> "ingredient 24",
    "Ingredient 25"=> "ingredient 25",
    "Ingredient 26"=> "ingredient 26",
    "Ingredient 27"=> "ingredient 27",
    "Ingredient 28"=> "ingredient 28",
    "Ingredient 29"=> "ingredient 29",
    "Ingredient 30"=> "ingredient 30",
    "Ingredient 31"=> "ingredient 31",
    "Ingredient 32"=> "ingredient 32",
    "Ingredient 33"=> "ingredient 33",
    "Ingredient 34"=> "ingredient 34",
    "Ingredient 35"=> "ingredient 35",
    "Ingredient 36"=> "ingredient 36",
    "Ingredient 37"=> "ingredient 37",
    "Ingredient 38"=> "ingredient 38",
    "Ingredient 39"=> "ingredient 39",
    "Ingredient 40"=> "ingredient 40",
    "Ingredient 41"=> "ingredient 41",
    "Ingredient 42"=> "ingredient 42",
    "Ingredient 43"=> "ingredient 43",
    "Ingredient 44"=> "ingredient 44",
    "Ingredient 45"=> "ingredient 45",
    "Ingredient 46"=> "ingredient 46",
    "Ingredient 47"=> "ingredient 47",
    "Ingredient 48"=> "ingredient 48",
    "Ingredient 49"=> "ingredient 49",
    "Ingredient 50"=> "ingredient 50",
    "Ingredient 51"=> "ingredient 51",
    "Ingredient 52"=> "ingredient 52",
    "Ingredient 53"=> "ingredient 53",
    "Ingredient 54"=> "ingredient 54",
    "Ingredient 55"=> "ingredient 55",
    "Ingredient 56"=> "ingredient 56",
    "Unnamed=> 62"=> "unnamed=> 62",
    "Unnamed=> 63"=> "unnamed=> 63",
    "Unnamed=> 64"=> "unnamed=> 64",
    "Unnamed=> 65"=> "unnamed=> 65",
    "Unnamed=> 66"=> "unnamed=> 66"};
    let ingredient_mapping: HashMap<String, String> = hmap! {"COSING Ref No" => "COSING Ref No",
    "INCI name"=> "INCI name",
    "INN name"=> "INN name",
    "Ph. Eur. Name"=> "Ph. Eur. Name",
    "CAS No"=> "CAS No",
    "EC No"=> "EC No",
    "NamexCas"=> "NamexCas",
    "NamexEC"=> "NamexEC",
    "NamexName"=> "NamexName",
    "Mix"=> "Mix",
    "Anexo.iii.name"=> "Anexo.iii.name",
    "Anexo.iii.EC"=> "Anexo.iii.EC",
    "Anexo.iii.CAS"=> "Anexo.iii.CAS",
    "Chem/IUPAC Name / Description"=> "Chem/IUPAC Name / Description",
    "nchar"=> "nchar",
    "Synonyms.formatx"=> "synonyms",
    "-"=> "-",
    "Name to Compare \"Tool\" (Risk databases)"=> "actual_name",
    "Restriction"=> "Restriction",
    "Function"=> "Function",
    "Anexo.iii.Criteria"=> "Anexo.iii.Criteria",
    "Info para Reporte"=> "info_para_reporte",
    "Update Date"=> "Update Date",
    "Observaciones"=> "Observaciones",
    "Citas"=> "cita",
    "Group.Cancer"=> "Group.Cancer",
    "Ref.Cancer"=> "Ref.Cancer",
    "Volume.Cancer"=> "Volume.Cancer",
    "Year.Cancer"=> "Year.Cancer",
    "Add Info.Cancer"=> "Add Info.Cancer",
    "Add Info.Dev"=> "Add Info.Dev",
    "Ref.Dev"=> "Ref.Dev",
    "Group.Endoc"=> "Group.Endoc",
    "Ref.Toxicity.Allergies"=> "Ref.Toxicity.Allergies",
    "Add Info.Toxicity.Allergies"=> "Add Info.Toxicity.Allergies",
    "Add Info.Total/partial use restrictions"=> "Add Info.Total/partial use restrictions",
    "Ref.Endoc"=> "Ref.Endoc",
    "Ref.Total/partial use restrictions"=> "Ref.Total/partial use restrictions",
    "Ref.Env"=> "Ref.Env",
    "Add Info.Env"=> "Add Info.Env",
    "Cancer.Risk"=> "cancer_risk",
    "Development.Risk"=> "development_risk",
    "Allergies.Risk"=> "allergies_risk",
    "Endocryne.Risk"=> "endocryne_risk",
    "Prohibited.Risk"=> "prohibited_risk",
    "Env.Risk"=> "env_risk",
    "Total.Risk"=> "total_risk"};
    return match source {
        Source::Ingredients => ingredient_mapping,
        Source::Products => product_mapping,
    };
}
