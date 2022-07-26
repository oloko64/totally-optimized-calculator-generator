def cabecalho():
    f = open("Calc.py", "w+")

    f.write("print({}Bem vindo a calculadora MK I{})\n".format('"', '"'))

    f.write("\nnum1 = input({}Digite o primeiro valor: {})\n".format('"', '"'))
    f.write("sign = input({}Digite o operador (+, -, *, /): {})\n".format('"', '"'))
    f.write("num2 = input({}Digite o segundo valor: {})\n\n".format('"', '"'))

    f.write("num1 = int(num1)\n"
            "num2 = int(num2)\n\n")

    f.close()


def inicio(operador, maximo):
    n1 = 0
    n2 = 0
    res = 0
    f = open("Calc.py", "a+")
    while n2 <= maximo:
        for i in range(maximo + 1):
            if operador == "+":
                res = n1 + n2
            elif operador == "-":
                res = n1 - n2
            elif operador == "/":
                if n2 == 0:
                    res = 0
                else:
                    res = n1 / n2
            else:
                res = n1 * n2

            f.write(
                "if num1 == {} and sign == {}{}{} and num2 == {}:\n    print({}{}{}{} = {}{})\n\n".format(n1, '"',
                                                                                                          operador, '"',
                                                                                                          n2,
                                                                                                          '"', n1,
                                                                                                          operador, n2,
                                                                                                          res, '"'))
            i = i + 1
            n1 = n1 + 1
        n1 = 0
        n2 = n2 + 1
    n1 = 0
    n2 = 0
    i = 0
    res = 0

    f.close()


def final():
    f = open("Calc.py", "a+")
    f.write("")
    f.write("input({}Pressione ENTER para sair{})".format('"', '"'))
    f.close()


if __name__ == '__main__':
    valor = 100

    cabecalho()

    inicio("+", valor)
    inicio("-", valor)
    inicio("*", valor)
    inicio("/", valor)

    final()
