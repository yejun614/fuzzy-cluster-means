from sklearn.datasets import make_blobs

X, y = make_blobs(n_samples = 500, n_features = 2, centers = 3, random_state = 1)

fp = open("data_blobs.csv", "w")
fp.write("X,Y\n")

for i in range(300):
  fp.write(f"{X[i][0]},{X[i][1]}\n")

fp.close()
