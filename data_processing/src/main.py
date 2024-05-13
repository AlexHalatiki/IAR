import pandas as pd
from sklearn.preprocessing import OneHotEncoder

def one_hot_encoding(coluna_especifica, df):
    encoder = OneHotEncoder(sparse_output=False)
    
    one_hot_encoded = encoder.fit_transform(df[[coluna_especifica]])
    
    colunas_dummy = encoder.get_feature_names_out([coluna_especifica])

    one_hot_df = pd.DataFrame(one_hot_encoded, columns=colunas_dummy, index=df.index)
    
    for coluna in colunas_dummy:
        one_hot_df[coluna] = one_hot_df[coluna].astype(int)

    df.drop(columns=[coluna_especifica], inplace=True)

    return pd.concat([df, one_hot_df], axis=1)
    

dados = pd.read_csv('./data/Airbnb_Data.csv')

dados.drop(columns=['NAME', 'country', 'host name', 'last review', 'license'], inplace=True)

dados.dropna(subset=['host_identity_verified', 'neighbourhood group', 'neighbourhood', 'lat', 'long', 'instant_bookable', 'cancellation_policy', 'room type', 'Construction year', 'number of reviews', 'reviews per month', 'review rate number', 'price'], inplace=True)

dados['Construction year'] = dados['Construction year'].astype(int)
dados['number of reviews'] = dados['number of reviews'].astype(int)
dados['review rate number'] = dados['review rate number'].astype(int)

dados['minimum nights'] = dados['minimum nights'].fillna(0).astype(int)
dados = dados[dados['minimum nights'] >= 0]

dados['calculated host listings count'] = dados['calculated host listings count'].fillna(1).astype(int)

dados['availability 365'] = dados['availability 365'].fillna(0).astype(int)
dados = dados[dados['availability 365'] >= 0]

dados['service fee'] = dados['service fee'].fillna(0).astype(int)

corrections = {
    'brookln': 'Brooklyn',
}

dados['neighbourhood group'] = dados['neighbourhood group'].replace(corrections)

dados = one_hot_encoding('cancellation_policy', dados)
dados = one_hot_encoding('room type', dados)

dados['price'] = dados['price'].apply(lambda x: (x + "0").replace(',', '') if ',' in x and len(x) == 4 else x.replace(',', ''))

dados.to_csv('./data/Airbnb_Data_Processed.csv', index=False)
