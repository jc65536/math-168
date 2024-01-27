
n = 10; % no. nodes
p = 0.5; % probability of keeping a particular edge

A = rand(n)<p; % generate random numbers, and keep the corresponding edge 
% only for random numbers less than p

A = triu(A,1); % keep only the upper triangular part of A
A = A+A.'; % turn it into a symmetric adjacency matrix

G = graph(A);

% now we make a vector that includes node centralities
% make a vector v(i), s.t. v(i) is the degree of node i
% degree of node i is the sum of all the entries in row i

v = sum(A,2); % sum over rows of A, we will add color labels to the nodes, representing the entries in v

plot(G,'NodeCData',v,'linewidth',2,'MarkerSize',10,'NodeFontSize',18); % colors the nodes according to values in v

colorbar;
set(gca,'fontsize',18);