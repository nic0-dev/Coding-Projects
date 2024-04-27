# Lecture 3: Measures of Information

Consider the following "informative" statements
1. The weather will be good next weekend
2. The weather was bad last sunday
3. You will get a passing grade in COE 161
4. You will pass all your subjects this Semester

> Idea: One gets information when learning something he/she/they is **uncertain** before

Recall: In Lecture 1, Shannon defined Information as
> **Information** is the **Resolution of Uncertainty**

Requirements for Information Measure
1. $\uparrow \#\ of\ outcomes\ \Rightarrow\ \uparrow\ Information$

2. Suppose A and B are INDEPENDENT information soruces:
   - **Additive:** Info(A and B) = Info(A) + Info(B) 
   - **Multiplicative:** #Outcomes (AB) = #Outcomes(A) * #Outcomes(B)

    **Initial guess:** Info(A) = log2 (#outcomes(A)) -> Base 2 unit = "bits"

    E.g. Number Guessing Game
    Q: What is the smallest # Yes/No Questions Needed to Identify $x\in[0,15]$?

    Consider: 

    $$x = 12 \rightarrow 1 1 0 0$$
    $$1st\ bit:\ x\ mod\ \geq 8?$$
    $$2nd\ bit:\ x\ mod\ 8 \geq 4?$$
    $$3rd\ bit:\ x\ mod\ 4 \geq 2?$$
    $$4th\ bit:\ x\ mod\ 2 = 1?$$

## Join Entropy

**Want:** Information Content of *Two or more RVs*

**Join Entropy:** Average Content of Multiple RVs

$$H(X,Y) = \sum_{(x,y)\in\mathcal{A}_X\cdot{}\mathcal{A}_Y}{P(x,y)\cdot log_2\frac{1}{P(x,y)}}\Rightarrow \ (2\ RVs)$$

$$H(\bar{X}) = \sum_{\bar{x}\in\mathcal{A}_{\bar X}}{P(\bar{x})\cdot log\frac{1}{P(\bar{x})}}\Rightarrow \ (General\ Case)$$
