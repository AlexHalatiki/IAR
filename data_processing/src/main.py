import pandas as pd
from sklearn.preprocessing import OneHotEncoder

# Funcao para aplicar One Hot Encoding 
def one_hot_encoding(coluna_especifica, df):
    encoder = OneHotEncoder(sparse_output=False)
    
    one_hot_encoded = encoder.fit_transform(df[[coluna_especifica]])
    
    colunas_dummy = encoder.get_feature_names_out([coluna_especifica])

    one_hot_df = pd.DataFrame(one_hot_encoded, columns=colunas_dummy, index=df.index)
    
    for coluna in colunas_dummy:
        one_hot_df[coluna] = one_hot_df[coluna].astype(int)

    df.drop(columns=[coluna_especifica], inplace=True)

    return pd.concat([df, one_hot_df], axis=1)
    
# Lendo base de dados
dados = pd.read_csv('./data/Airbnb_Data.csv')

# Excluindo atributos irrelevantes
dados.drop(columns=['id', 'NAME', 'host id', 'host name', 'neighbourhood', 'country', 'country code', 'host name', 'last review', 'house_rules', 'license'], inplace=True)

# Excluindo e tratando valores nulos
dados.dropna(subset=['host_identity_verified', 'neighbourhood group', 'lat', 'long', 'instant_bookable', 'cancellation_policy', 'room type', 'Construction year', 'number of reviews', 'reviews per month', 'review rate number', 'price'], inplace=True)

dados['Construction year'] = dados['Construction year'].astype(int)
dados['number of reviews'] = dados['number of reviews'].astype(int)
dados['review rate number'] = dados['review rate number'].astype(int)

dados['minimum nights'] = dados['minimum nights'].fillna(0).astype(int)
dados = dados[dados['minimum nights'] >= 0]

dados['calculated host listings count'] = dados['calculated host listings count'].fillna(1).astype(int)

dados['availability 365'] = dados['availability 365'].fillna(0).astype(int)
dados = dados[dados['availability 365'] >= 0]

dados['service fee'] = dados['service fee'].fillna(0).astype(int)

# Padronizacao
corrections = {
    'brookln': 'Brooklyn',
}

dados['neighbourhood group'] = dados['neighbourhood group'].replace(corrections)

# Aplicando One Hot Encoding
dados = one_hot_encoding('host_identity_verified', dados)
dados = one_hot_encoding('neighbourhood group', dados)
dados = one_hot_encoding('instant_bookable', dados)
dados = one_hot_encoding('cancellation_policy', dados)
dados = one_hot_encoding('room type', dados)

# Tratando "Outliers"
dados['price'] = dados['price'].apply(lambda x: (x + "0").replace(',', '') if ',' in x and len(x) == 4 else x.replace(',', ''))
dados['long'] = dados['long'].astype(str).apply(lambda x: x[:3] + '.' + x[3:].replace('.', ''))

# Movendo price pro final
cols = [col for col in dados.columns if col != 'price'] + ['price']
dados = dados[cols]

# Salvando a base de dados processada
dados.to_csv('./data/Airbnb_Data_Processed.csv', index=False)
